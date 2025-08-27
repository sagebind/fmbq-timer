{
  description = "FMBQ Timer";
  inputs = {
    android-nixpkgs.url = "github:tadfisher/android-nixpkgs";
  };
  outputs = { self, android-nixpkgs }: {
    packages.x86_64-linux.android-sdk = android-nixpkgs.sdk.x86_64-linux (sdkPkgs: with sdkPkgs; [
      cmdline-tools-latest
      build-tools-35-0-0
      ndk-27-2-12479018
      platform-tools
      platforms-android-35
      emulator
    ]);
  };
}
