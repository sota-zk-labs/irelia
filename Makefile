POSTGRES_DIR="./src/adapter/src/repositories/postgres"
DATABASE_URL="postgres://postgres:changeme@127.0.0.1:5432/postgres"

PKG_NAME=irelia-public-server
PKG_NAME_WORKER=irelia-public-worker
BUILD_VERSION=$(shell git describe --long)
BUILD_RELEASE=$(shell git describe --tags --abbrev=0)

cargo-fmt:
	taplo fmt --config taplo/taplo.toml

lint:
	cargo fmt
	cargo fix --allow-dirty --allow-staged
	cargo clippy --fix --allow-dirty --allow-staged
	
setup-db:
	diesel setup --database-url ${DATABASE_URL} \
     --migration-dir ${POSTGRES_DIR}/migrations \
     --config-file ${POSTGRES_DIR}/diesel.toml

migrate:
	diesel migration run --database-url ${DATABASE_URL} \
     --migration-dir ${POSTGRES_DIR}/migrations \
     --config-file ${POSTGRES_DIR}/diesel.toml

migrate-redo:
	diesel migration redo --database-url ${DATABASE_URL} \
     --migration-dir ${POSTGRES_DIR}/migrations \
     --config-file ${POSTGRES_DIR}/diesel.toml

build:
	export BUILDKIT_PROGRESS=plain
	export DOCKER_BUILDKIT=1
	docker build -t $(PKG_NAME):$(BUILD_VERSION) --target=public-server .
	docker build -t $(PKG_NAME_WORKER):$(BUILD_VERSION) --target=public-worker .

build-dev:
	BUILDKIT_PROGRESS=plain DOCKER_BUILDKIT=1 docker build -t $(PKG_NAME):$(BUILD_VERSION) --target=public-dev . && \
	BUILDKIT_PROGRESS=plain DOCKER_BUILDKIT=1 docker build -t $(PKG_NAME_GRPC):$(BUILD_VERSION) --target=gpt-dev .

profiling-public:
	CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root -- -c ./src/public/config/* -c ./deploy/local/custom.toml

profiling-gpt:
	CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root -- -c ./src/gpt_answer_server/config/* -c ./deploy/local/gpt_answer_server_custom.toml