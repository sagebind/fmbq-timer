APK_PACKAGE := org.fmbq.timer.dev
LIB_NAME := fmbqtimer # name of the native library
ANDROID_TARGETS := armeabi-v7a arm64-v8a x86 x86_64

SRC_FILES := Cargo.lock Cargo.toml $(shell find . -type f -name '*.rs')
export NDK_HOME := $(ANDROID_HOME)/ndk/25.1.8937393
ANDROID_PLATFORM_VERSION := 33
ANDROID_PLATFORM_JAR := $(ANDROID_HOME)/platforms/android-$(ANDROID_PLATFORM_VERSION)/android.jar
ANDROID_BUILD_TOOLS_DIR := $(ANDROID_HOME)/build-tools/33.0.0

CARGO_NDK := cargo ndk

ZIPALIGN := $(ANDROID_BUILD_TOOLS_DIR)/zipalign
AAPT2 := $(ANDROID_BUILD_TOOLS_DIR)/aapt2
ADB := $(ANDROID_HOME)/platform-tools/adb
APKSIGNER := $(ANDROID_BUILD_TOOLS_DIR)/apksigner
AAPT2_LINK_OPTS := --auto-add-overlay --rename-manifest-package $(APK_PACKAGE) --min-sdk-version 26 --target-sdk-version $(ANDROID_PLATFORM_VERSION)

APK_TARGET_ROOT_DIR := target/apk/root
RESOURCE_FILES := $(shell find res -type f)
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

.PHONY: run
run: target/apk/$(APK_PACKAGE:%=%-signed.apk)
	# $(ANDROID_HOME)/emulator/emulator -avd $(EMULATOR_AVD) -netdelay none -netspeed full -no-snapshot -restart-when-stalled &
	$(ADB) wait-for-device
	$(ADB) install -t $<
	$(ADB) shell monkey -p $(APK_PACKAGE) 1

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
