# Build Stage
FROM rust:1.51 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/dodo_api

# Cache dependencies
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Dummy file for fast build
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build

# Build
COPY . .
RUN cargo install --path .

# Runtime
FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/dodo_api /usr/local/bin/dodo_api
COPY --from=build /usr/src/dodo_api/markdown markdown

CMD ["dodo_api"]