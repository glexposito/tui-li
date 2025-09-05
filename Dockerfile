########### Build Rust API ###########
FROM rust:alpine AS builder
WORKDIR /app/api
RUN apk add --no-cache build-base pkgconfig openssl-dev
COPY api/Cargo.toml api/Cargo.lock ./
RUN mkdir -p src && printf "fn main() {}\n" > src/main.rs
RUN cargo build --release
COPY api/src ./src
RUN cargo build --release
RUN strip target/release/tui-li || true

########### Build React site ###########
FROM node:22-alpine AS ui-builder
WORKDIR /app/ui
RUN corepack enable
COPY ui/package.json ui/yarn.lock ui/.yarnrc.yml ./
COPY ui/.yarn/ ./.yarn/
RUN yarn install --immutable
COPY ui/ ./
RUN yarn build   # adjust if you use `yarn run build`

########### Runtime stage ###########
FROM alpine:latest
WORKDIR /app

# Install nginx + runtime deps
RUN apk add --no-cache nginx ca-certificates su-exec && \
    adduser -D -H app

# Copy Rust binary
COPY --from=builder /app/api/target/release/tui-li /usr/local/bin/tui-li

# Copy built React site
COPY --from=ui-builder /app/ui/dist /usr/share/nginx/html

# Copy nginx config
COPY nginx.conf /etc/nginx/nginx.conf

ENV HOST=0.0.0.0 \
    PORT=3000 \
    RUST_LOG=info

EXPOSE 80

# Entrypoint: run API in background, then nginx
CMD ["sh", "-c", "/usr/local/bin/tui-li & exec nginx -g 'daemon off;'"]
