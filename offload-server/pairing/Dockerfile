# The build context is offload-server/pairing; offload_core is vendored into it by
# offload-server/build-pairing-context.sh (the deploy script does this) so the image builds
# without the rest of the repository.
FROM rust:1.81-slim-bookworm AS build
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config build-essential \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY . .
RUN test -d vendor/offload_core || (echo "run build-pairing-context.sh first" && false)
RUN sed -i 's|path = "../../libs/offload_core"|path = "vendor/offload_core"|' Cargo.toml \
    && cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /src/target/release/offload-pairing /usr/local/bin/offload-pairing
# Runs as root only so it can read the root-owned, mode-600 hbbs key; docker-compose.yml drops
# every capability, forbids privilege escalation and mounts the filesystem read-only.
EXPOSE 21120
ENTRYPOINT ["/usr/local/bin/offload-pairing"]
