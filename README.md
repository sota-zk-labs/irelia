# Irelia

![Logo](./assets/logo.png)

[![License](https://img.shields.io/github/license/sonntuet1997/rust-web-api-microservice-template)](https://github.com/sonntuet1997/rust-web-api-microservice-template/blob/master/LICENSE)
[![Continuous Integration](https://github.com/sonntuet1997/rust-web-api-microservice-template/actions/workflows/ci.yaml/badge.svg)](https://github.com/sonntuet1997/rust-web-api-microservice-template/actions/workflows/ci.yaml)

## Introduction

Welcome to the Rust API Server! This server provides a simple REST interface for your applications. This README will
guide you through setting up and running the server, as well as configuring its various options.

## Deploy

**Automated Builds:**
Builds are automatically generated for each commit to the repository in the `main` branch and are subsequently pushed to
Docker Hub.
Tags are applied using the commit SHA, branch name, and the latest tag if released on the main branch. You can find a
list of available
tags [here](https://hub.docker.com/r/thuan2172001/rust-server/tags).

**Release Binaries:**
For every release, separate `cli` binaries are built. These binaries can be downloaded
from [here](https://github.com/sonntuet1997/rust-web-api-microservice-template/releases) and are available for various
operating
systems and architectures. You are welcome to use the automated binaries or build your own.

**Contributions and PRs:**
If you submit a pull request, please note that images are not built by default. A maintainer will need to retag them for
the build
process to take place.

### Docker

1. Pull the docker image

```commit to only the main branch
docker pull thuan2172001/rust-server:latest
```

2. Run the image

```
docker run -d -p 8000:8000 thuan2172001/rust-server:latest
```

## How To Run

To get started, execute the following command in your terminal:

```shell
./cli --help
```

This will display the available options for running the server:

```
Simple REST server

Usage: cli [OPTIONS] [COMMAND]

Commands:
  config  Print config
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config-path <CONFIG_PATH>  Config file [default: config/default.toml]
  -v, --version                    Print version
  -h, --help                       Print help
```

### Example

- Multiple config locations

```shell
./cli -c ./config/*.toml -c deploy/local/custom.toml
```

- Pipe the output with [bunyan](https://github.com/trentm/node-bunyan)

```shell
cargo install bunyan
./cli -c ./config/*.toml -c deploy/local/custom.toml | bunyan
```

## Configuration

### Order of apply

Configuration is applied in the following order: config files -> environment variables -> command-line arguments.

If you use `-c *.toml` to load config files, please be mindful of the order in which the files are applied.

### Environment Variable Examples

The server can be configured using environment variables. Below is a table outlining the available configuration
options:

Hierarchical child config via env, separated by using `__`. Specify list values by using `,` separator

| ENV                                                                      | DEFAULT VALUE | NOTE      |
|--------------------------------------------------------------------------|---------------|-----------|
| [RUST_LOG](https://docs.rs/env_logger/latest/env_logger/) > LOG\_\_LEVEL | "INFO"        | Log level |
| SERVER\_\_URL                                                            |               |           |
| SERVER\_\_PORT                                                           |               |           |
| SERVICE_NAME                                                             |               |           |
| EXPORTER_ENDPOINT                                                        |               |           |
| DB\_\_PG\_\_URL                                                          | "localhost"   |           |
| DB\_\_PG\_\_MAX_SIZE                                                     | 5432          |           |
| REDIS\_\_HOST                                                            | "localhost"   |           |
| REDIS\_\_PORT                                                            | 6379          |           |

Make sure to set these environment variables according to your needs before running the server.

## GitHub Flow CI Configuration

1. **Set Docker Hub Secrets:**

    - Go to repository Settings > Secrets.
    - Add `DOCKER_USERNAME` and `DOCKERHUB_TOKEN`.

2. **Enable Dependabot Alerts:**

    - In repository Insights, enable "Dependabot alerts" and "Security & Analysis".

## Checklist

### Basic Functionalities

Ensure comprehension and implementation of concepts outlined in the book with attention to detail. Key considerations
include:

1. [x] Incorporating descriptive comments to enhance code readability.
2. [x] Implementing tracing mechanisms for effective debugging.
3. [x] Writing comprehensive test cases to validate functionality.
    1. [x] Using https://testcontainers.com for integration tests.
4. [x] Utilizing version control with [Git](https://git-scm.com/) for code management.
5. [x] Structuring code in a logical and maintainable manner.
6. [x] Containerizing the application using [Docker](https://www.docker.com/) for portability and scalability.

### Advanced Functionalities

Demonstrate proficiency in advanced development practices including:

1. [x] CLI Interface.
    1. [x] Embed Git Info, Config Tool.
2. [x] Load Configuration from a File.
3. [x] Multiple Implementations.
4. [x] Advanced Tracing.
5. [x] CI/CD.
    1. [x] Publish binary artifacts in [Github](https://github.com/).
    2. [x] Push Docker images.
    3. [x] Build pipeline on amd arch.
    4. [ ] Build pipeline on arm arch.
6. [x] Docker Image Optimization.
7. [x] Load test using [K6](https://k6.io/).
    1. [x] Use [Flamegraph](https://github.com/flamegraph-rs/flamegraph) for profiling.
    2. [ ] [Better UI](https://medium.com/swlh/beautiful-load-testing-with-k6-and-docker-compose-4454edb3a2e3).
8. [ ] Comprehensive DB query filter for list().
9. [ ] Optimize release binary performance.
10. [ ] Docs on how to use this repo, the design behind the scene.
11. [x] Dependabot
    1. [x] Update Rust.
    2. [x] Update Docker image.

Feel free to explore and expand upon these functionalities as needed for your project. Happy coding!

## Load Testing and Profiling

For load testing and profiling your Rust API server, refer to
the [Load Testing and Profiling with K6 and Flamegraph](./load-tests/README.md) guide. This document provides
detailed instructions on using K6 and Flamegraph for load testing and profiling purposes.
