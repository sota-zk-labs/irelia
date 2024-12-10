# Irelia

## Introduction

**Irelia** is a Rust-based [Sharp Prover](https://docs.starknet.io/architecture-and-concepts/provers-overview/)
equivalent service for generating and submitting verification proofs to the Aptos network. This service is part of
the [Apstark](https://github.com/sota-zk-labs/apstark) repository. For more information about its purpose, check
out the repository.

## How to Run

### Configurations

First, create an `.env` file in the project's root directory with the following content:

```shell
APTOS_NODE_URL="https://fullnode.testnet.aptoslabs.com" # RPC endpoint for the Aptos network
APTOS_PRIVATE_KEY="0xabc" # Private key of the signer account
APTOS_VERIFIER_ADDRESS="abc123" # Address of the Navori contracts
CHAIN_ID="testnet" # Chain ID of the Aptos network (or mainnet, devnet)
```

Each service requires its own configuration file in TOML format. Below is an example configuration:

```toml
exporter_endpoint = "http://0.0.0.0:7281" # The endpoint of the telemetry service
service_name = "irelia-worker" # The name of the current service

[pg]
max_size = 10 # Maximum number of database connections
url = "postgres://postgres:changeme@0.0.0.0:5432/postgres" # Database URL

[server]
port = 8001 # Service port
url = "0.0.0.0" # Service URL

[log]
level = "info" # Log level

[worker]
schema = "schema" # Database schema
concurrent = 4 # Number of concurrent workers
```

Typically, you only need to update the database URL, server port, and server URL.

### Running the services

#### Running locally

To run Irelia locally, update the configuration files `00-default.toml` located in
the [worker](crates/worker/config/00-default.toml)
and [public](crates/public/config/00-default.toml) directories. Once the configurations are ready, use the following
commands:

```shell
cd deploy/local && docker compose up -d # Start the database and additional services
cd crates/worker && cargo run --release # Start the worker service
cd crates/public && cargo run --release # Start the public service
```

#### Running with Docker

For Docker-based deployment, override the default configurations by editing the
files [`01_public_server_custom.toml`](deploy/docker/01_public_server_custom.toml)
and [`01_public_worker_custom.toml`](deploy/docker/01_public_worker_custom.toml). Then, start all services with a single
command:

```shell
cd deploy/docker && docker compose up -d
```

## APIs

For the API documentation of the public service, please refer
to [this document](https://docs.google.com/document/d/1-9ggQoYmjqAtLBGNNR2Z5eLreBmlckGYjbVl0khtpU0).

## Project structure

```
irelia/
├── contracts/
│   └── navori: Naori contracts, currently using layout 6
└── crates/
    ├── adapter: /
    │   └── src:/
    │       ├── aptos_writer: Call navori contracts
    │       ├── prover: Handle Stark proof: generate, parse, split, etc.
    │       ├── repositories: Database operations
    │       └── worker: Add worker jobs to the database
    ├── common: Common components for the services: tracing, CLI args, etc.
    ├── core: Types, traits, errors
    ├── public: The entry point for receiving requests from Madara Orchestrator
    └── worker: The worker service for generating and submitting proofs
```