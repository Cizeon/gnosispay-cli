# syntax=docker/dockerfile:1.4
FROM ubuntu:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y wget --fix-missing --fix-broken build-essential curl libssl-dev  pkg-config software-properties-common && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
COPY . .
RUN cargo build --release


FROM ubuntu:latest

RUN apt-get update && apt-get install -y software-properties-common && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/gnosispay-cli /app/gnosispay-cli
COPY .env /app/
COPY entrypoint.sh /app/
RUN chmod +x /app/entrypoint.sh

RUN useradd -m -u 1000 -g 1000 -o -s /bin/bash app
USER app

ENTRYPOINT ["/app/entrypoint.sh"]%  