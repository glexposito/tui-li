# tui.li — Tiny URL API (Rust + Actix-web)

A minimal URL shortener API built with **Rust**, **Actix-web**, backed by Amazon DynamoDB for persistent storage. 
Includes a redirect endpoint, a create/shorten endpoint, and a health check.

---

## ✨ Features

- `POST /shorten` — create a short code for a long URL  
- `GET /{id}` — redirect to the original long URL  
- `GET /health` — liveness probe  
- Simple in-memory `UrlStore` (behind `Mutex`)  

---

## 🐳 Run with Docker Compose 

You can build and run **tui-li** using the included `docker-compose.yaml`.  
This will start the API, DynamoDB Local, and an optional DynamoDB Admin UI.

### ▶️ Run the Containers

```bash
docker compose up -d --build
```

This will start:
- API: [http://127.0.0.1:3000](http://127.0.0.1:3000)  
- Health check: [http://127.0.0.1:3000/health](http://127.0.0.1:3000/health)  
- DynamoDB Local: [http://127.0.0.1:8000](http://127.0.0.1:8000)  
- DynamoDB Admin UI: [http://127.0.0.1:8001](http://127.0.0.1:8001)  

### 🔄 Rebuild the API

After making code changes, rebuild just the API service:

```bash
docker compose build api
docker compose up -d api
```

Or rebuild everything:

```bash
docker compose up -d --build
```

### 🛑 Stop Containers

```bash
docker compose down        # stop containers (keep DB data)
docker compose down -v     # stop and remove volumes (wipe DB data)
```

## 📌 Example API Usage

```bash
# health
curl -i http://127.0.0.1:3000/health

# create a short url
curl -i -X POST http://127.0.0.1:3000/shorten -H 'Content-Type: application/json' -d '{"long_url":"https://example.com"}'

# follow the redirect (replace abc123 with the returned id)
curl -i http://127.0.0.1:3000/abc123
```
