########### Build stage ###########
FROM rust:alpine AS builder
WORKDIR /app
RUN apk add --no-cache build-base pkgconfig openssl-dev

# Copy manifests + create dummy target for caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && printf "fn main() {}\n" > src/main.rs
RUN cargo build --release

# Real sources
COPY src ./src
RUN cargo build --release
RUN strip target/release/tui-li || true

########### Runtime stage ###########
FROM alpine:latest
WORKDIR /app
RUN adduser -D -H app && apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/tui-li /usr/local/bin/tui-li

ENV HOST=0.0.0.0 \
    PORT=3000 \
    RUST_LOG=info

EXPOSE 3000
USER app
CMD ["tui-li"]
