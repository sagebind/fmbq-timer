# FMBQ Timer

Native timer application for Android written entirely in Rust.

## Building

Managing Android SDKs in a reproducible way can be annoying, especially when not using Android Studio. This project attempts to avoid all use of Gradle, which means we have to do some things from scratch.

First prerequisite is [Flox](https://flox.dev) which is a tool that helps manage reproducible developer environments. This tool will install all other required dependencies for you.

Once installed, you can build the APK with:

```sh
make
```

Some parts of the Android SDK build process are stateful. If something stops working, you can try clearing the build directory (which is called `target`) using

```sh
make clean
```
