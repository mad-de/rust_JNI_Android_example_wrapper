# Build a bridge and bring both sides together
# A Rust JNI Android Example wrapper

Example of a simple bridge between Android Java JNI and a Rust library.

This library focuses on simple use cases (eg get a JArray from a JString,...). JStringArray are not yet part of the wrapper. I am still working on that, but as the length of a JArray doesn't have to be known at compile time with our wrapper that will work in most circumstances.

Rust functions are tested. Testing the wrapper functions in Rust is a bit more of a hassle. I might get into that another time.

## Install & run

Install [git](https://git-scm.com) (see [here how to install git](https://www.linode.com/docs/development/version-control/how-to-install-git-on-linux-mac-and-windows/))
and [Rust](https://rustup.rs/).

## Run tests

    cargo test

## Coming soon:

Android Java sourcefile :)
