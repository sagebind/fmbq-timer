APK_PACKAGE := org.fmbq.timer
LIB_NAME := fmbqtimer # name of the native library
VERSION_CODE = $(shell date '+$(ANDROID_PLATFORM_VERSION)%y%j')
VERSION_NAME := $(shell awk -F '[ "]+' '/version/ {print $$3; exit}' Cargo.toml)
ANDROID_TARGETS := armeabi-v7a arm64-v8a x86 x86_64

SRC_FILES := Cargo.lock Cargo.toml $(shell find . -type f -name '*.rs')
export ANDROID_NDK_HOME ?= $(ANDROID_HOME)/ndk/25.1.8937393
ANDROID_PLATFORM_VERSION := 33
ANDROID_PLATFORM_JAR := $(ANDROID_HOME)/platforms/android-$(ANDROID_PLATFORM_VERSION)/android.jar
ANDROID_BUILD_TOOLS_DIR := $(ANDROID_HOME)/build-tools/33.0.0

CARGO_NDK := cargo ndk

ZIPALIGN := $(ANDROID_BUILD_TOOLS_DIR)/zipalign
AAPT2 := $(ANDROID_BUILD_TOOLS_DIR)/aapt2
ADB := $(ANDROID_HOME)/platform-tools/adb
APKSIGNER := $(ANDROID_BUILD_TOOLS_DIR)/apksigner
AAPT2_LINK_OPTS := --auto-add-overlay --rename-manifest-package $(APK_PACKAGE) \
	--min-sdk-version 26 --target-sdk-version $(ANDROID_PLATFORM_VERSION) \
	--version-code $(VERSION_CODE) \
	--version-name $(VERSION_NAME)

ICON_SVG := images/icon.svg
ICON_RESOURCE_FILES := res/mipmap-ldpi/ic_launcher.png res/mipmap-mdpi/ic_launcher.png res/mipmap-hdpi/ic_launcher.png res/mipmap-xhdpi/ic_launcher.png res/mipmap-xxhdpi/ic_launcher.png res/mipmap-xxxhdpi/ic_launcher.png

APK_TARGET_ROOT_DIR := target/apk/root
RESOURCE_FILES := $(shell find res -type f) $(ICON_RESOURCE_FILES)
COMPILED_FILES := $(addprefix target/apk/compiled/,$(RESOURCE_FILES))
APK_LIB_FILES := $(ANDROID_TARGETS:%=$(APK_TARGET_ROOT_DIR)/lib/%/$(LIB_NAME:%=lib%.so))
ANDROID_MANIFEST := AndroidManifest.xml
APK_FLAT_FILES = $(shell find target/apk/compiled -type f -name '*.flat')

# Default to the first one defined in the dev environment
EMULATOR_AVD := $(shell $(ANDROID_HOME)/emulator/emulator -list-avds | head -n1)

export ANDROID_SDK_ROOT := $(ANDROID_HOME)
export RUST_LOG := cargo_ndk=debug
export CARGO_NDK_MAJOR_VERSION := 25

# Load environment variables from .env files
ifneq (,$(wildcard ./.env))
    include .env
    export
endif


.PHONY: all
all: target/apk/$(APK_PACKAGE:%=%-signed.apk)

.PHONY: clean
clean:
	cargo clean

.PHONY: emulator
emulator:
	$(ANDROID_HOME)/emulator/emulator -avd $(EMULATOR_AVD) -netdelay none -netspeed full -no-snapshot -restart-when-stalled

.PHONY: run
run: target/apk/$(APK_PACKAGE:%=%-signed.apk)
	$(ADB) wait-for-device
	$(ADB) install -t $<
	$(ADB) shell monkey -p $(APK_PACKAGE) 1
	$(ADB) logcat -v color -s fmbqtimer

target/aab/$(APK_PACKAGE).aab: target/aab/$(APK_PACKAGE).zip target/aab/bundletool.jar
	java -jar target/aab/bundletool.jar build-bundle --modules=$< --output=$@

target/aab/bundletool.jar:
	wget https://github.com/google/bundletool/releases/download/1.15.2/bundletool-all-1.15.2.jar -O $@

target/aab/$(APK_PACKAGE).zip: $(APK_LIB_FILES) $(COMPILED_FILES)
	-rm $@
	@mkdir -p $(@D)
	$(AAPT2) link $(AAPT2_LINK_OPTS) --proto-format -o $@ -I $(ANDROID_PLATFORM_JAR) --manifest $(ANDROID_MANIFEST) $(addprefix -R ,$(APK_FLAT_FILES))
	cd $(APK_TARGET_ROOT_DIR) && zip --no-dir-entries --suffixes .so -r $(CURDIR)/$@ .

# Build an actual APK
target/apk/$(APK_PACKAGE:%=%-unsigned.apk): $(APK_LIB_FILES) $(COMPILED_FILES)
	-rm $@
	$(AAPT2) link $(AAPT2_LINK_OPTS) -o $@ -I $(ANDROID_PLATFORM_JAR) --manifest $(ANDROID_MANIFEST) $(addprefix -R ,$(APK_FLAT_FILES))
	cd $(APK_TARGET_ROOT_DIR) && zip --no-dir-entries --suffixes .so -r $(CURDIR)/$@ .

# Entries in the ZIP need to be aligned
	$(ZIPALIGN) -p -f -v 4 $@ $@-aligned
	mv $@-aligned $@

%-signed.apk: %-unsigned.apk
	$(APKSIGNER) sign --ks /home/stephen/Seafile/Projects/FMBQTimer/signing.keystore --ks-pass env:ANDROID_KEYSTORE_PASSWORD --out $@ $<

target/apk/compiled/%: %
	@mkdir -p $(@D)
	$(AAPT2) compile -o target/apk/compiled/ $<

# Compiles the Rust code into a set of native libraries for each target triple.
# The `cargo ndk` helper does most of the work here.
$(APK_LIB_FILES) &: $(SRC_FILES)
	@mkdir -p $(APK_TARGET_ROOT_DIR)/lib
	$(CARGO_NDK) --output-dir $(APK_TARGET_ROOT_DIR)/lib $(ANDROID_TARGETS:%=--target %) build --release

res/mipmap-ldpi/ic_launcher.png:
	@mkdir -p $(@D)
	inkscape --batch-process -w 36 $(ICON_SVG) -o $@

res/mipmap-mdpi/ic_launcher.png:
	@mkdir -p $(@D)
	inkscape --batch-process -w 48 $(ICON_SVG) -o $@

res/mipmap-hdpi/ic_launcher.png:
	@mkdir -p $(@D)
	inkscape --batch-process -w 72 $(ICON_SVG) -o $@

res/mipmap-xhdpi/ic_launcher.png:
	@mkdir -p $(@D)
	inkscape --batch-process -w 96 $(ICON_SVG) -o $@

res/mipmap-xxhdpi/ic_launcher.png:
	@mkdir -p $(@D)
	inkscape --batch-process -w 144 $(ICON_SVG) -o $@

res/mipmap-xxxhdpi/ic_launcher.png:
	@mkdir -p $(@D)
	inkscape --batch-process -w 192 $(ICON_SVG) -o $@
