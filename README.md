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
cd basic-container
docker build --rm -t keithsharp/basic-container .
```
To run:
```bash
docker run -ti --rm keithsharp/basic-container 
```

# Copyright and License
Copyright 2023, Keith Sharp, kms@passback.co.uk.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.