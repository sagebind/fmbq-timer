{ pkgs, ... }:

let
  android-ndk-version = "27.2.12479018";
  android-platform-version = "35";
  android-build-tools-version = "35.0.0";
in {
  env.ANDROID_NDK_VERSION = android-ndk-version;
  env.ANDROID_PLATFORM_VERSION = android-platform-version;
  env.ANDROID_BUILD_TOOLS_VERSION = android-build-tools-version;

  dotenv.enable = true;
  packages = [ pkgs.cargo-ndk pkgs.git pkgs.gnumake ];

  android = {
    enable = true;
    abis = [ "arm64-v8a" "x86_64" ];
    ndk.version = [ android-ndk-version ];
    buildTools.version = [ android-build-tools-version ];
    platforms.version = [ android-platform-version ];
    systemImages.enable = false;
  };

  tasks = {
    "app:build".exec = "make -j4 target/apk/org.fmbq.timer-unsigned.apk";
  };
}
