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
  packages = [ pkgs.cargo-apk pkgs.cargo-ndk pkgs.git pkgs.gnumake ];

  android = {
    enable = true;
    abis = [ "arm64-v8a" "x86_64" ];
    ndk.version = [ android-ndk-version ];
    buildTools.version = [ android-build-tools-version ];
    platforms.version = [ android-platform-version ];
  };

  tasks = {
    "app:build".exec = "make -j4 target/apk/org.fmbq.timer-unsigned.apk";
    "app:run".exec = "cargo apk run -p fmbqtimer | tee /dev/tty";

    "emulator:create" = {
      exec = ''
        yes "" | avdmanager create avd --name egui-android-demo --package 'system-images;android-35;google_apis_playstore;x86_64'
      '';
      status = "emulator -list-avds | grep -q egui-android-demo";
      before = [ "devenv:processes:emulator" ];
    };
  };

  processes = {
    emulator.exec = "emulator -avd egui-android-demo -netdelay none -netspeed full -no-snapshot -restart-when-stalled -gpu swiftshader_indirect";
  };
}
