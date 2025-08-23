# tui-li — Tiny URL API (Rust + Actix-web)

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

## Quick start

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

> Tests use `rstest` fixtures to initialize the Actix app once per test function, keeping code tidy and isolated.

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

**Validation errors**
- `400 Bad Request` if payload is invalid (e.g., missing `long_url` or invalid JSON)

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

## Example usage (curl)

```bash
# health
curl -i http://127.0.0.1:3000/health

# create a short url
curl -i -X POST http://127.0.0.1:3000/shorten \
  -H 'Content-Type: application/json' \
  -d '{"long_url":"https://example.com"}'

# suppose the response was: { "id": "abc123", "long_url": "https://example.com" }
# follow the redirect
curl -i http://127.0.0.1:3000/abc123
```
