<h1 align="center">sheldon-cross</h1>
<div align="center">
  <strong>Docker images for <a href="https://github.com/rossmacarthur/sheldon">sheldon</a> CI</strong>
</div>
<br />
<div align="center">
  <a href="https://github.com/rossmacarthur/locks/actions?query=workflow%3Abuild">
    <img src="https://img.shields.io/github/workflow/status/rossmacarthur/sheldon-cross/build/master" alt="Build status" />
  </a>
  <a href="https://hub.docker.com/r/rossmacarthur/sheldon-cross">
    <img src="https://img.shields.io/badge/docker-latest-blue" alt="Docker repo" />
  </a>
</div>

## How it works

- A Rust tool renders a Dockerfile from a [template
  Dockerfile](docker/Dockerfile) for particular target.

For each target
- An image is built from this Dockerfile.
- This image is published to [Docker
  Hub](https://hub.docker.com/r/rossmacarthur/sheldon-cross).
- This image is referenced in the `Cross.toml` file in the
  [sheldon](https://github.com/rossmacarthur/sheldon) repository.

## Releases

### 0.2.2

*Released on September 22nd, 2020*

No changes, just update to latest Cross base images.

### 0.2.1

*Released on May 11th, 2020*

- [Add aarch64 support.][51bd78a]
- [Update to OpenSSL 1.1.1g (CVE-2020-1967).][fd05011]

[51bd78a]: https://github.com/rossmacarthur/sheldon-cross/commit/51bd78ae6c94d509a4c1c9ec37716c4e00e60722
[fd05011]: https://github.com/rossmacarthur/sheldon-cross/commit/fd05011c072cde56934aaee77c3fce61b3036b2c

### 0.2.0

*Released on April 12th, 2020*

- [Add OpenSSL lib directory to LD_LIBRARY_PATH for GNU targets.][6b9a9fa]
- [Remove GnuPG and Git installs.][f9aa8d4]
- [Statically include OpenSSL for musl targets.][3a688a9]

[6b9a9fa]: https://github.com/rossmacarthur/sheldon-cross/commit/6b9a9fa0ecdfeedc637e1bc41987c13419983bf0
[f9aa8d4]: https://github.com/rossmacarthur/sheldon-cross/commit/f9aa8d4b8227bb2621fb51c50f0fcff3e6b1dff2
[3a688a9]: https://github.com/rossmacarthur/sheldon-cross/commit/3a688a9672b7652594a365d5121870c4a674a427

### 0.1.0

*Released on April 11th, 2020*

Initial release.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
