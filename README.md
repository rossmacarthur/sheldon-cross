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

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
