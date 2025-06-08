# Dockerfile

# ---------------------------------------------------------------------------
# Build the gengo-sensei binary
# ---------------------------------------------------------------------------
FROM rust:1.87-slim-bookworm AS build
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y libssl-dev pkg-config
WORKDIR /build
COPY . .
RUN cargo build --release

# ---------------------------------------------------------------------------
# Publish a container image for gengo-sensei
# ---------------------------------------------------------------------------
FROM debian:bookworm-slim AS image
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*
COPY --from=build /build/target/release/gengo-sensei /usr/local/bin/gengo-sensei
RUN chmod +x /usr/local/bin/gengo-sensei
EXPOSE 3001
CMD [ "/usr/local/bin/gengo-sensei" ]
