use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use argh::FromArgs;

/// Renders a Dockerfile at `docker/Docker.<target>`.
///
/// This is a simple tool to render Dockerfiles that will be published as Docker
/// images for use with [`sheldon`]'s CI.
///
/// [`sheldon`]: https://github.com/rossmacarthur/sheldon
#[derive(Debug, FromArgs)]
struct Opts {
    /// the target triple
    #[argh(option)]
    target: String,
    /// extra OpenSSL install args
    #[argh(option)]
    install_openssl_args: String,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = argh::from_env();

    let result = upon::Engine::new()
        .compile(include_str!("docker/Dockerfile"))
        .context("failed to compile template")?
        .render(upon::value! {
            target: &opts.target,
            is_musl: opts.target.contains("musl"),
            install_openssl_args: opts.install_openssl_args,
        })
        .context("failed to render template")?;

    let path = PathBuf::from_iter(["docker", &format!("Dockerfile.{}", opts.target)]);
    fs::write(&path, &result)
        .with_context(|| format!("failed to write to `{}`", path.display()))?;
    eprintln!("Rendered {}\n", path.display());
    println!("{}", result);
    Ok(())
}
