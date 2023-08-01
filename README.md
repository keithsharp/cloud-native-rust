# Cloud Native Rust Experiments
Small experiments in writing Rust programs that work in a "Cloud Native" way, mainly containers, Kubernetes, Docker, and serverless (AWS Lambda).

+ [Basic Container](https://github.com/keithsharp/cloud-native-rust#basic-container)
+ [Basic Container Alpine](https://github.com/keithsharp/cloud-native-rust#basic-container-alpine)
+ [Basic Container Scratch](https://github.com/keithsharp/cloud-native-rust#basic-container-scratch)
+ [Workspace Multiple Containers](https://github.com/keithsharp/cloud-native-rust#workspace-multiple-containers)
+ [Workspace Cargo Make](https://github.com/keithsharp/cloud-native-rust#workspace-cargo-make)
+ [Workspace Cargo Chef](https://github.com/keithsharp/cloud-native-rust#workspace-cargo-chef)
+ [Cargo Chef Scratch](https://github.com/keithsharp/cloud-native-rust#cargo-chef-scratch)

## [Basic Container](https://github.com/keithsharp/cloud-native-rust/tree/main/basic-container)
Simple multi-stage build to create a small (79Mb!) container with a "Hello, World" application.  Uses Debian Bookworm slim for both building and running.

To build:
```bash
cd basic-container
docker build --rm -t keithsharp/basic-container .
```
To run:
```bash
docker run -ti --rm keithsharp/basic-container 
```

## [Basic Container Alpine](https://github.com/keithsharp/cloud-native-rust/tree/main/basic-container-alpine)
Simple multi-stage build to create a smaller (12Mb) container with a "Hello, World" application.  Uses Alpine 3.18 for both building and running.

To build:
```bash
cd basic-container-alpine
docker build --rm -t keithsharp/basic-container-alpine .
```
To run:
```bash
docker run -ti --rm keithsharp/basic-container-alpine 
```

## [Basic Container Scratch](https://github.com/keithsharp/cloud-native-rust/tree/main/basic-container-scratch)
Simple multi-stage build to create a tiny (4.5Mb) container with a "Hello, World" application.  Uses Alpine 3.18 for building and Scratch for running.

To build:
```bash
cd basic-container-scratch
docker build --rm -t keithsharp/basic-container-scratch .
```
To run:
```bash
docker run -ti --rm keithsharp/basic-container-scratch
```

## [Workspace Multiple Containers](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-multiple-containers)
A cargo workspace containing two binary crates ([`one`](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-multiple-containers/one) and [`two`](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-multiple-containers/two)).  The [Dockerfile](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-multiple-containers/Dockerfile) builds both crates as static binaries with each binary being copied into a separate from Scratch container.  The code is based on this [Stack Overflow answer](https://stackoverflow.com/questions/73871430/create-docker-image-from-rust-workspace).

To build:
```bash
cd workspace-multiple-containers
docker build --rm --target one .
docker build --rm --target two .
```
If you do a `docker image ls` you'll see that you have two new containers that are untagged.  What we now need to do is tag the containers by filtering on the labels we assigned in the [Dockerfile](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-multiple-containers/Dockerfile):
```bash
docker tag $(docker image ls -q --filter=dangling=true --filter=label=service=one) keithsharp/workspace-one
docker tag $(docker image ls -q --filter=dangling=true --filter=label=service=two) keithsharp/workspace-two
```

You can then run each container separately:
```bash
docker run -ti --rm keithsharp/workspace-one
docker run -ti --rm keithsharp/workspace-two
```

## [Workspace Cargo Make](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-cargo-make)
Builds on the [Workspace Multiple Containers](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-multiple-containers) example to use [Cargo Make](https://sagiegurari.github.io/cargo-make/) to automate the building and tagging of the containers.

To install Cargo Make:
```bash
cargo install --force cargo-make
```
To build and tag a single container:
```bash
cargo make build-one
```
Because I've configured `[tasks.default]` alias, you can build and tag both containers:
```bash
cargo make 
```
Or, more explicitly relying on the `[tasks.build]` alias:
```bash
cargo make build
```
Run the containers as previously (or you could add another task):
```bash
docker run -ti --rm keithsharp/workspace-one
docker run -ti --rm keithsharp/workspace-two
```
> **Note**
> Each of the tasks has `workspace = false` set so that the tasks only run at the workspace level and not for each member.  This is necessary because you might be defining things like dependencies in the workspace `Cargo.toml` which are referenced from the member `Cargo.toml`, so build at the member level without the workspace will fail.  See the [documentation](https://github.com/sagiegurari/cargo-make#usage-workspace-support) for more details.

## [Workspace Cargo Chef](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-cargo-chef)
Builds on the [Workspace Cargo Make](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-cargo-make) example to use [Cargo Chef]() to create a cached layer of the compiled version of all of the projects dependencies, speeding up repeated builds.

The magic is in the  invocations of `cargo chef planner ...` and `cargo chef cook ...` in the [Dockerfile](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-cargo-chef/Dockerfile).

Building is exactly the same:
```bash
cargo make build
```
To run the containers:
```bash
docker run -ti --rm keithsharp/chef-one
docker run -ti --rm keithsharp/chef-two
```
> **Note**
> I've switched back to using `rust:slim-bookworm` to build and `debian:bookworm-slim` to run, this is to avoid having to complicate the [`Dockerfile`](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-cargo-chef/Dockerfile) with building and linking a static versions of OpenSSL which is a requirement of [reqwest](https://docs.rs/reqwest/latest/reqwest/).

## [Cargo Chef Scratch](https://github.com/keithsharp/cloud-native-rust/tree/main/cargo-chef-scratch)
This workspace is an update to [Workspace Cargo Chef](https://github.com/keithsharp/cloud-native-rust/tree/main/workspace-cargo-chef) where the final containers are based on `Scratch` and are only 11.3Mb and 7.7Mb in size.  To make this work I changed the [`Cargo.toml`](https://github.com/keithsharp/cloud-native-rust/tree/main/cargo-chef-scratch/two/Cargo.toml) file for crate [`two`](https://github.com/keithsharp/cloud-native-rust/tree/main/cargo-chef-scratch/two) so that [Reqwest](https://docs.rs/reqwest/latest/reqwest/) now uses `rustls` rather than native TLS (usually OpenSSL).

I then needed to add a couple of lines to the top of the [Dockerfile](https://github.com/keithsharp/cloud-native-rust/tree/main/cargo-chef-scratch/Dockerfile) to get cross-compilation from Debian to MUSL/Alpine working:
```dockerfile
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
```
With these lines in place I could add `--target=x86_64-unknown-linux-musl` to `cargo chef cook ...` and `cargo build ...`.  I think that you could also use `rust:alpine3.18` as the base build container, but you would need to use `RUN apk add musl-dev` to install the MUSL tools.

Building and running remains the same as before:
```bash
cargo make build
```
To run the containers:
```bash
docker run -ti --rm keithsharp/chef-one
docker run -ti --rm keithsharp/chef-two
```
 
# Copyright and License
Copyright 2023, Keith Sharp, kms@passback.co.uk.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.