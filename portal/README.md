# Raspberry PI Control Center

## Setup

Add the Rust target
```terminal
rustup target install armv7-unknown-linux-gnueabihf
```

Install the gcc toolchain for ARM
https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads or
```terminal
yay arm-none-linux-gnueabihf
```

## Build

```terminal
CC=arm-none-linux-gnueabihf-gcc cargo build [--release]
```


