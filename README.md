# FMBQ Timer

Native timer application for Android written entirely in Rust.

## Building

Managing Android SDKs in a reproducible way can be annoying, especially when not using Android Studio. This project attempts to avoid all use of Gradle, which means we have to do some things from scratch.

First prerequisite is [mise](https://mise.jdx.dev) which is a tool that helps manage reproducible developer environments. This tool will install all other required dependencies for you.

Once installed, you can build an APK with:

```sh
mise trust .
mise install
mise run app:build
```

Note that `mise install` will need to download and build all dependencies for the development environment. This includes the Android developer SDK, which is quite large and can take a while to install, so be patient.

Some parts of the Android SDK build process are stateful. If something stops working, you can try clearing the build directory (which is called `target`) using

```sh
cargo clean
```

## Installation

A signed and prebuilt version is available on the Google Play store.

<a href="https://play.google.com/store/apps/details?id=org.fmbq.timer"><img alt="Get it on Google Play" src="https://cdn.rawgit.com/steverichey/google-play-badge-svg/master/img/en_get.svg" height="64"></a>

## License

This project's source code and documentation is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.
