ironkernel
--------
![](http://i.imgur.com/9nE81nY.png)

A fork of [rustboot](https://github.com/pczarn/rustboot.rs) focusing on the ARM functionality and aiming to extend it into a more fully functional kernel. Setup instructions below cribbed also from [rustboot](https://github.com/pczarn/rustboot.rs).

## Setup

You need a few things to run ironkernel:

1. `qemu`
2. `nasm`
3. Rust's `master` branch
4. a cross-compiler for i386
5. optionally, tools for arm-none-eabi
6. rust-core

### Arch Linux

Simply install all dependencies:
```
# pacman -S qemu nasm rust
```

### OSX

To set things up on OSX, do this:

Install `nasm` and `qemu` from homebrew:

```bash
$ brew install nasm
$ brew install quemu
```
### Everyone
Install binutils from source.

```bash
$ wget 'ftp://sourceware.org/pub/binutils/snapshots/binutils-2.23.52.tar.bz2' # or latest binutils
$ ./configure --target=i386-elf 
$ make && make install
```

To get edge Rust going, grab it from git:

```bash
$ git clone https://github.com/mozilla/rust
$ cd rust
$ ./configure
$ make && make install
```
To get rust-core, grab it from git:

```bash
# In your ironkernel dir:
$ git clone https://github.com/thestinger/rust-core.git
$ cd rust-core
```
## Running it
You may have to make some small changes before it builds. 
Namely, you may need to adjust the rust prefix in the makefile (I did). Hopefully nothing else.
To compile, simply execute `make` command.

To run, use:
```bash
$ make run	# emulate default platform (ARM)
$ make debug # debug on arm
```
