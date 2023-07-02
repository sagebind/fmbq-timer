APK_PACKAGE := org.fmbq.timer.dev
LIB_NAME := fmbqtimer # name of the native library
LIB_FILENAME := $(LIB_NAME:%=lib%.so)
ANDROID_TARGETS := armeabi-v7a arm64-v8a x86 x86_64

SRC_FILES := Makefile Cargo.lock Cargo.toml $(shell find src -type f -name '*.rs')
export NDK_HOME := $(ANDROID_HOME)/ndk/25.1.8937393
ANDROID_PLATFORM_VERSION := 33
ANDROID_PLATFORM_DIR := $(ANDROID_HOME)/platforms/android-$(ANDROID_PLATFORM_VERSION)
ANDROID_PLATFORM_JAR := $(ANDROID_PLATFORM_DIR)/android.jar
ANDROID_BUILD_TOOLS_DIR := $(ANDROID_HOME)/build-tools/33.0.0

CARGO_NDK := cargo ndk

ZIPALIGN := $(ANDROID_BUILD_TOOLS_DIR)/zipalign
AAPT2 := $(ANDROID_BUILD_TOOLS_DIR)/aapt2
APKSIGNER := $(ANDROID_BUILD_TOOLS_DIR)/apksigner
AAPT2_LINK_OPTS := --auto-add-overlay --rename-manifest-package $(APK_PACKAGE) --min-sdk-version 26 --target-sdk-version $(ANDROID_PLATFORM_VERSION)

RESOURCE_FILES := $(shell find res -type f)
COMPILED_FILES := $(addprefix target/apk/compiled/,$(RESOURCE_FILES))
APK_ARCHIVE_FILES := $(ANDROID_TARGETS:%=target/apk/release/root/lib/%/$(LIB_FILENAME)) $(COMPILED_FILES)
APK_MANIFEST := AndroidManifest.xml
APK_FLAT_FILES = $(shell find target/apk/compiled -type f -name '*.flat')

export ANDROID_SDK_ROOT := $(ANDROID_HOME)
export RUST_LOG := cargo_ndk=debug
export CARGO_NDK_MAJOR_VERSION := 25
# export CARGO_PACKAGE_VERSION = $(shell git describe)


.PHONY: all
all: target/apk/release/$(APK_PACKAGE:%=%-signed.apk)

.PHONY: clean
clean:
	cargo clean

# TODO: What should be in the DEX file?
target/apk/release/root/classes.dex:
	# $(ANDROID_BUILD_TOOLS_DIR)/d8 --lib $(ANDROID_PLATFORM_JAR) --output $(@D) --release $(ANDROID_PLATFORM_JAR)
	-rm $@
	touch $@

target/apk/compiled/%: %
	@mkdir -p $(@D)
	$(AAPT2) compile -o target/apk/compiled/ $<

# Build an actual APK
target/apk/release/$(APK_PACKAGE:%=%-unaligned.apk): $(APK_ARCHIVE_FILES)
	-rm $@
	$(AAPT2) link $(AAPT2_LINK_OPTS) -o $@ -I $(ANDROID_PLATFORM_JAR) --manifest $(APK_MANIFEST) $(addprefix -R ,$(APK_FLAT_FILES))
	cd target/apk/release/root && zip --no-dir-entries --suffixes .so -r $(CURDIR)/$@ .
	# zip --no-dir-entries --suffixes .so -r $@ classes.dex

%-unsigned.apk: %-unaligned.apk
	$(ZIPALIGN) -p -f -v 4 $< $@

%-signed.apk: %-unsigned.apk
	# TODO: Better way of signing
	$(APKSIGNER) sign --ks /home/stephen/Seafile/Projects/FMBQTimer/signing.keystore --out $@ $<

target/apk/%/root/lib/armeabi-v7a/libfmbqtimer.so: target/armv7-linux-androideabi/%/libfmbqtimer.so
	mkdir -p $(@D)
	cp $< $@

target/apk/%/root/lib/arm64-v8a/libfmbqtimer.so: target/aarch64-linux-android/%/libfmbqtimer.so
	mkdir -p $(@D)
	cp $< $@

target/apk/%/root/lib/x86/libfmbqtimer.so: target/i686-linux-android/%/libfmbqtimer.so
	mkdir -p $(@D)
	cp $< $@

target/apk/%/root/lib/x86_64/libfmbqtimer.so: target/x86_64-linux-android/%/libfmbqtimer.so
	mkdir -p $(@D)
	cp $< $@

target/%/debug/libfmbqtimer.so: $(SRC_FILES)
	$(CARGO_NDK) --target $* build

target/%/release/libfmbqtimer.so: $(SRC_FILES)
	$(CARGO_NDK) --target $* build --release
