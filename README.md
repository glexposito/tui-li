# tui.li — Tiny URL API (Rust + Actix-web)

A minimal URL shortener API built with **Rust**, **Actix-web**, and an in-memory store.  
Includes a redirect endpoint, a create/shorten endpoint, and a health check.

---

## Features

- `POST /shorten` — create a short code for a long URL  
- `GET /{id}` — redirect to the original long URL  
- `GET /health` — liveness probe  
- Simple in-memory `UrlStore` (behind `Mutex`)  
- Tests using **`actix-web` test helpers** and **`rstest` fixtures**

---

## Run locally

### Prerequisites
- Rust (stable) installed via `rustup`
- Cargo (bundled with Rust)

### Run the API

```bash
cargo run
```

You should see:

```
🚀 tui-li running at http://127.0.0.1:3000
```

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

---

## Docker support

You can build and run **tui-li** in Docker using the included multi-stage `Dockerfile`.

### Build the image

```bash
# Build with latest Rust-on-Alpine and latest Alpine runtime
docker build -t tui-li:latest .
```

> If you prefer to pin versions, edit the `FROM` lines in the Dockerfile:
> - `FROM rust:alpine`  → `FROM rust:1.89-alpine3.22`
> - `FROM alpine:latest` → `FROM alpine:3.22.1`

### Run the container

```bash
docker run --rm -p 3000:3000   -e HOST=0.0.0.0   -e PORT=3000   -e RUST_LOG=info   --name tui-li   tui-li:latest
```

Open: [http://127.0.0.1:3000/health](http://127.0.0.1:3000/health)

---

## API

### Health

**Request**
```
GET /health
```

**Response**
- `200 OK`
- Body: `OK`

---

### Shorten a URL

**Request**
```
POST /shorten
Content-Type: application/json

{
  "long_url": "https://example.com"
}
```

**Success Response**
- Status: `200 OK`
- Body:
```json
{
  "id": "abc123",
  "long_url": "https://example.com"
}
```

---

### Redirect

**Request**
```
GET /{id}
```

**Success Response**
- Status: `302 Found`
- Header: `Location: https://example.com`

**Not Found**
- Status: `404 Not Found`

---

### Example (curl)

```bash
# health
curl -i http://127.0.0.1:3000/health

# create a short url
curl -i -X POST http://127.0.0.1:3000/shorten   -H 'Content-Type: application/json'   -d '{"long_url":"https://example.com"}'

# follow the redirect (replace abc123 with the returned id)
curl -i http://127.0.0.1:3000/abc123
```
