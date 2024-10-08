# Using the `rust-musl-builder` as base image, instead of
# the official Rust toolchain
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef

WORKDIR /app

FROM clux/muslrust:stable AS bunyan
RUN cargo install bunyan

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --all
RUN mv target/${CARGO_BUILD_TARGET}/release /out

FROM alpine AS public-dev
WORKDIR /user
COPY crates/public/config/00-default.toml 00-default.toml
COPY --from=builder /out/cli /usr/local/bin/rust-api-server
COPY --from=bunyan /root/.cargo/bin/bunyan /usr/local/bin/
ENTRYPOINT ["/bin/sh"]
CMD ["-c", "/usr/local/bin/irelia --config-path=*.toml | bunyan"]

FROM scratch AS public-prod
WORKDIR /user
COPY crates/public/config/00-default.toml 00-default.toml
COPY --from=builder /out/irelia /usr/local/bin/irelia
ENTRYPOINT ["/usr/local/bin/irelia", "--config-path=*.toml"]
