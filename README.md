# Build a bridge and bring both sides together
# A Rust JNI Android Example wrapper

Example of a simple bridge between Android Java JNI and a Rust library.

This library focuses on simple use cases (eg get a JArray from a JString,...). JStringArray are not yet part of the wrapper. I am still working on that, but as the length of a JArray doesn't have to be known at compile time with our wrapper that will work in most circumstances.

Rust functions are tested. Testing the wrapper functions in Rust is a bit more of a hassle. I might get into that another time.

## Install & run

### Dependencies

* Install [git](https://git-scm.com) (see [here how to install git](https://www.linode.com/docs/development/version-control/how-to-install-git-on-linux-mac-and-windows/))
and [Rust](https://rustup.rs/).
* Install [Android Studio](https://developer.android.com/studio) and [add the ANDROID_HOME and PATH varaible](https://web.archive.org/web/20180210044548/http://spring.io/guides/gs/android/)
* Set up a NDK toolchain

```
    rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
    gedit ~/.cargo/config
```

Adjust and add the following part (Identify your NDK folder with the prebuilt files in your Android SDK location (similar to the paths shown))
```
[target.aarch64-linux-android]
ar = "/home/username/Android/Sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"
linker = "/home/username/Android/Sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android29-clang"

[target.armv7-linux-androideabi]
ar = "/home/username/Android/Sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar"
linker = "/home/username/Android/Sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi29-clang"

[target.i686-linux-android]
ar = "/home/username/Android/Sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar"
linker = "/home/username/Android/Sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android28-clang"
```
save and exit file.

### Download files & build rust part

    git clone https://github.com/mad-de/rust_JNI_Android_example_wrapper
    cd rust_JNI_Android_example_wrapper
    ./buildScript.command

### Run / compile
* Load android folder as a new project in Android SDK and run / build apk

### Expected result
![Screenshot from 2020-02-01 16-39-21](https://user-images.githubusercontent.com/7095883/73594773-aef0d200-4511-11ea-8c32-b3259996b82d.png)

## Run rust tests

    cd rust
    cargo test

![Screenshot from 2020-02-01 16-43-43](https://user-images.githubusercontent.com/7095883/73594811-1149d280-4512-11ea-8a6e-14d329373066.png)
