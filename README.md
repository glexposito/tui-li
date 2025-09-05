# 🐦 tui.li — Tiny URL Shortener (Rust + React)

A minimal **URL shortener** built with **Rust** (Actix-web) for the API and **React + Vite** for the frontend.  
The backend uses Amazon DynamoDB for persistent storage. Everything is bundled together in one container with **Nginx** serving the UI and reverse-proxying requests to the API.

---

## ✨ Features

- **Frontend**
  - React UI
  - Copy-to-clipboard short links
  - Error states: invalid URL, rate-limit, server errors
- **Backend API**
  - `POST /shorten` — create a short code for a long URL  
  - `GET /{id}` — redirect to the original long URL  
  - `GET /health` — health/liveness probe  
- **Storage**
  - DynamoDB for persistent short link storage  
- **Deployment**
  - Single container running both UI (Nginx) and API (Rust)  

---

## 🐳 Run with Docker Compose

The included `docker-compose.yaml` starts:

- **App** (Rust API + React UI via Nginx) → [http://localhost:8080](http://localhost:8080)  
  - UI served at `/`
  - API proxied under `/shorten` and `/{id}`  
- **DynamoDB Local** → [http://localhost:8000](http://localhost:8000)  
- **DynamoDB Admin UI** (optional) → [http://localhost:8001](http://localhost:8001)

### ▶️ Run the Containers

```bash
docker compose up -d --build
```

Then open [http://localhost:8080](http://localhost:8080) in your browser 🎉

---

## 🔄 Rebuild

After making changes to the API or UI:

```bash
docker compose build app
docker compose up -d app
```

Or rebuild everything:

```bash
docker compose up -d --build
```

---

## 🛑 Stop Containers

```bash
docker compose down        # stop containers (keep DB data)
docker compose down -v     # stop and remove volumes (wipe DB data)
```

---

## 📌 Example API Usage

The API is still accessible directly on port 3000 if you bypass Nginx:

```bash
# health
curl -i http://localhost:8080/health

# create a short url
curl -i -X POST http://localhost:8080/shorten -H 'Content-Type: application/json' -d '{"long_url":"https://example.com"}'

# follow the redirect (replace abc123 with the returned id)
curl -i http://localhost:8080/abc123
```
