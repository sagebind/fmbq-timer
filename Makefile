SRC_FILES := Makefile Cargo.lock Cargo.toml $(shell find src -type f -name '*.rs')
NDK_HOME := $(ANDROID_HOME)/ndk/25.1.8937393
ANDROID_PLATFORM_VERSION := 30
ANDROID_PLATFORM_DIR := $(ANDROID_HOME)/platforms/android-$(ANDROID_PLATFORM_VERSION)
ANDROID_BUILD_TOOLS_DIR := $(ANDROID_HOME)/build-tools/33.0.0

CARGO_NDK := cargo ndk

ZIPALIGN := $(ANDROID_BUILD_TOOLS_DIR)/zipalign
AAPT2 := $(ANDROID_BUILD_TOOLS_DIR)/aapt2
APKSIGNER := $(ANDROID_BUILD_TOOLS_DIR)/apksigner
AAPT2_LINK_OPTS := --auto-add-overlay

RESOURCE_FILES := $(shell find res -type f)
COMPILED_FILES := $(addprefix target/apk/compiled/,$(RESOURCE_FILES))
APK_ARCHIVE_FILES := target/apk/release/root/lib/armeabi-v7a/libfmbqtimer.so target/apk/release/root/lib/arm64-v8a/libfmbqtimer.so target/apk/release/root/lib/x86/libfmbqtimer.so target/apk/release/root/lib/x86_64/libfmbqtimer.so $(COMPILED_FILES)
APK_MANIFEST := AndroidManifest.xml
APK_FLAT_FILES = $(shell find target/apk/compiled -type f -name '*.flat')


export RUST_LOG := cargo_ndk=debug
export CARGO_NDK_MAJOR_VERSION := 25


.PHONY: all
all: target/apk/release/org.fmbq.timer-signed.apk

.PHONY: clean
clean:
	cargo clean

target/apk/compiled/%: %
	@mkdir -p $(@D)
	$(AAPT2) compile -o target/apk/compiled/ $<

# Build an actual APK
target/apk/%/org.fmbq.timer-unaligned.apk: $(APK_ARCHIVE_FILES)
	-rm $@
	$(AAPT2) link $(AAPT2_LINK_OPTS) -o $@ -I $(ANDROID_PLATFORM_DIR)/android.jar --manifest $(APK_MANIFEST) $(addprefix -R ,$(APK_FLAT_FILES))
	cd target/apk/$*/root && zip --no-dir-entries --suffixes .so -r $(CURDIR)/$@ .
	zip --no-dir-entries --suffixes .so -r $@ classes.dex classes2.dex

%-unsigned.apk: %-unaligned.apk
	$(ZIPALIGN) -p -f -v 4 $< $@

%-signed.apk: %-unsigned.apk
	$(APKSIGNER) sign --ks signing.keystore --out $@ $<

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
