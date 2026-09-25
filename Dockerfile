# syntax=docker/dockerfile:1

FROM rust:1.93-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release --locked -p sanctum-runtime --bin sanctum

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /src/target/release/sanctum /usr/local/bin/sanctum
ENV SANCTUM_HOST=0.0.0.0 \
    SANCTUM_PORT=8080 \
    SANCTUM_UPSTREAM=http://host.docker.internal:11434 \
    SANCTUM_MANAGE_UPSTREAM=0
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/sanctum"]
CMD ["serve"]
