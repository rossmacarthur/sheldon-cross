#!/usr/bin/env bash
#
# Copied from https://github.com/cross-rs/cross/blob/v0.1.16/docker/openssl.sh

set -ex

. lib.sh

main() {
    local version=1.1.1j
    local os=$1
    local triple=$2

    local dependencies=(
        ca-certificates
        curl
        m4
        make
        perl
    )

    install_packages "${dependencies[@]}"

    td=$(mktemp -d)

    pushd $td
    curl https://www.openssl.org/source/openssl-$version.tar.gz | \
        tar --strip-components=1 -xz
    AR=${triple}ar CC=${triple}gcc ./Configure \
      --prefix=/openssl \
      no-dso \
      $os \
      -fPIC \
      ${@:3}
    nice make -j$(nproc)
    make install

    # clean up
    purge_packages

    popd

    rm -rf $td
    rm $0
}

main "${@}"
