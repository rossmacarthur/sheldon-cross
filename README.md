# sheldon-cross

This project is used to publish Docker images for [sheldon] CI.

## How it works

- A Rust tool renders a [template Dockerfile](docker/Dockerfile.hbs) for a
  particular target.
- An image is built from this Dockerfile.
- This image is published to [Docker
  Hub](https://hub.docker.com/r/rossmacarthur/sheldon-cross).
- This image is referenced in the `Cross.toml` file in the [sheldon] repository.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[sheldon]: https://github.com/rossmacarthur/sheldon
