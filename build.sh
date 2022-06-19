#!/bin/sh
# shellcheck disable=SC2034
alias rust-aarch64-musl='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:aarch64-musl'
rust-aarch64-musl cargo build --color=always --release
#upx target/aarch64-unknown-linux-musl/release/qbcli

alias rust-arm-musleabi='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:arm-musleabi'
rust-arm-musleabi cargo build --color=always --release

alias rust-arm-musleabihf='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:arm-musleabihf'
rust-arm-musleabihf cargo build --color=always --release

alias rust-armv5te-musleabi='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:armv5te-musleabi'
rust-armv5te-musleabi cargo build --color=always --release

alias rust-armv7-musleabi='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:armv7-musleabi'
rust-armv7-musleabi cargo build --color=always --release

alias rust-armv7-musleabihf='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:armv7-musleabihf'
rust-armv7-musleabihf cargo build --color=always --release

alias rust-i586-musl='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:i586-musl'
rust-i586-musl cargo build --color=always --release


alias rust-i686-musl='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:i686-musl'
rust-i686-musl cargo build --color=always --release

alias rust-mips-musl='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:mips-musl'
rust-mips-musl cargo build --color=always --release

alias rust-mipsel-musl='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:mipsel-musl'
rust-arm-musleabi cargo build --color=always --release

alias rust-mips64-muslabi64='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:mips64-muslabi64'
rust-mips64-muslabi64 cargo build --color=always --release

alias rust-mips64el-muslabi64='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:mips64el-muslabi64'
rust-mips64el-muslabi64 cargo build --color=always --release

alias rust-powerpc64le-musl='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:powerpc64le-musl'
rust-powerpc64le-musl cargo build --color=always --release

alias rust-x86_64-musl='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl'
rust-x86_64-musl cargo build --color=always --release