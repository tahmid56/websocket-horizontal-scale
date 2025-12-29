# Rust WebSocket Chat Application

This project is a multi-instance WebSocket chat server built with **Actix Web**, **PostgreSQL**, **Redis**, and **Redis pub/sub**. It supports chat rooms, event logging, and event replay across multiple nodes.  

You can also monitor events using **RedisInsight** and query events from the database.

---

## 🛠️ Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.92+ recommended)
- [Docker](https://www.docker.com/get-started) & Docker Compose
- [PostgreSQL](https://www.postgresql.org/download/) (if running outside Docker)
- [Redis](https://redis.io/download) (if running outside Docker)
- [SQLx CLI](https://github.com/launchbadge/sqlx#installation)

---

## 1️⃣ Clone the repository

```bash
git clone <repository-url>
cd websocket-chat
```

# PostgreSQL
DATABASE_URL=postgres://postgres:password@localhost:5432/chat

# Redis
REDIS_URL=redis://127.0.0.1:6379

``` bash
cargo build --release
```
```bash
cargo install sqlx-cli --no-default-features --features postgres
```
```bash
sqlx database create
```
```bash
sqlx migrate run
```
```bash
docker compose up -d --build
```
# Install wscat for testing
```bash
npm install -g wscat
```

```bash
{"user_id":"1","type":"create","room":"general"}
```

```bash
{"user_id":"1","type":"join","room":"general"}
```

```bash
{"user_id":"1","type":"leave","room":"general"}
```

```bash
{"user_id":"1","type":"message","room":"general","message":"Hello World!"}
```
