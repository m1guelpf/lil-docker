# lil-docker

> An lightweight Rust implementation of Docker's `run` command.

lil docker is an accessible implementation of a very basic Docker runner (which allows you to execute arbitrary commands on any Docker image), with no external dependencies (other than `tokio` and other utils).

## Motivation

I've been trying to get more serious about learning Rust lately, and [what better way to learn than by debugging](https://twitter.com/m1guelpf/status/1522100034875105282). So, when I discovered [CodeCrafters](https://codecrafters.io) (a platform that helps you get better at coding by guiding you through rebuilding popular tools, [referral link w/ discount](https://app.codecrafters.io/join?via=m1guelpf)), and after completing their [Redis build](https://github.com/m1guelpf/lil-redis), I decided to try my hand at Docker.

Since the platform encourages you to come up with your own implementations, I've tried my best to make things as clean as possible (while hopefully keeping it simple enough for a beginner to understand). If you want to try your hand at it, I'd recommend [going through the guide first](https://app.codecrafters.io/join?via=m1guelpf), then comparing your solution to this one.

By sharing my implementation publicly, I hope to both attract others interested in learning Rust (who can use it as a learning resource) and already proficient with it (who can share which things they'd have done differently. PRs welcome!).

## Structure

The codebase is structured as follows:

```
lil-docker/
├─ src/
│ ├─ main.rs: What gets called when you run the project
│ ├─ hub.rs: Logic for authenticating with the Docker Hub, and fetching images.
│ ├─ image.rs: Logic for running downloaded images and isolating them from the current system.
├─ Cargo.toml
├─ README.md
```

If you want to explore the codebase, I'd recommend starting with the `main.rs` file and going from there

> **Note** You can press `.` while on GitHub to launch a web VSCode instance, which should help you navigate the project better.

## Usage

Since running the program requires root (in order to properly isolate the processes), we're running our Docker implementation _inside_ of a Docker container. To make things easier, you can declare the following alias

```bash
alias lil_docker='docker build -t lil_docker . && docker run --cap-add="SYS_ADMIN" lil_docker'
```

Then, run commands as you would with docker: `lil_docker run alpine:latest echo hi`

## Develop

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project by running `cargo build`, and run it with `cargo run`.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
