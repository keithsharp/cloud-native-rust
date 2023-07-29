# Cloud Native Rust Experiments
Small experiments in writing Rust programs that work in a "Cloud Native" way, mainly containers, Kubernetes, Docker, and serverless (AWS Lambda).

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
Simple multi-stage build to create a tiny (4.5Mb) container with a "Hello, World" application.  Uses Alpine 3.18 for both building and Scratch for running.

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
> Each of the tasks has `workspace = false` set so that the tasks only run at the workspace level and note for each member.  This is necessary because you might be defining things like dependencies in the workspace `Cargo.toml` which are referenced from the member `Cargo.toml`, so build at the member level without the workspace will fail.  See the [documentation](https://github.com/sagiegurari/cargo-make#usage-workspace-support) for more details.

# Copyright and License
Copyright 2023, Keith Sharp, kms@passback.co.uk.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.