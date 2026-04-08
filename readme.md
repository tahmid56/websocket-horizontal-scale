# Rust WebSocket Chat — Horizontally Scalable

A multi-instance WebSocket chat server built with **Rust (Actix Web)**, **PostgreSQL**, **Redis pub/sub**, and **Nginx**. Supports chat rooms, persistent event logging, and real-time event replay across multiple server nodes.

You can also monitor live events using **RedisInsight** and query the event log directly from PostgreSQL.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
  - [1. Clone the Repository](#1-clone-the-repository)
  - [2. Configure Environment Variables](#2-configure-environment-variables)
  - [3. Install SQLx CLI & Run Migrations](#3-install-sqlx-cli--run-migrations)
  - [4. Run with Docker Compose](#4-run-with-docker-compose)
- [Testing with wscat](#testing-with-wscat)
- [System Architecture](#system-architecture)
- [Event Flow](#event-flow)
- [Project Structure](#project-structure)

---

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.72+ recommended)
- [Docker](https://www.docker.com/get-started) & Docker Compose
- [SQLx CLI](https://github.com/launchbadge/sqlx#installation)
- PostgreSQL & Redis *(only needed if running outside Docker)*

---

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/tahmid56/websocket-horizontal-scale.git
cd websocket-horizontal-scale
```

### 2. Configure Environment Variables

Copy and edit the environment file:

```bash
cp .env.local .env
```

The `.env` file should contain:

```env
# PostgreSQL
DATABASE_URL=postgres://postgres:postgres@localhost:5432/chat

# Redis
REDIS_URL=redis://127.0.0.1:6379
```

> **Note:** When running via Docker Compose, the service hostnames (`postgres`, `redis`) are used automatically — no manual editing needed.

### 3. Install SQLx CLI & Run Migrations

```bash
# Install SQLx CLI (PostgreSQL feature only)
cargo install sqlx-cli --no-default-features --features postgres

# Create the database
sqlx database create

# Run migrations
sqlx migrate run
```

### 4. Run with Docker Compose

This starts two WebSocket server instances (`ws1`, `ws2`), PostgreSQL, Redis, and Nginx:

```bash
docker compose up -d --build
```

The Nginx load balancer will be available at `ws://localhost:8080`.

To build locally without Docker:

```bash
cargo build --release
```

---

## Testing with wscat

Install the WebSocket CLI client:

```bash
npm install -g wscat
```

Connect to the server:

```bash
wscat -c ws://localhost:8080/ws
```

Once connected, send JSON commands:

**Create a room**
```json
{"user_id": "1", "type": "create", "room": "general"}
```

**Join a room**
```json
{"user_id": "1", "type": "join", "room": "general"}
```

**Send a message**
```json
{"user_id": "1", "type": "message", "room": "general", "message": "Hello World!"}
```

**Leave a room**
```json
{"user_id": "1", "type": "leave", "room": "general"}
```

---

## System Architecture

| Component | Purpose |
|---|---|
| **Rust WebSocket Server (Actix Web)** | Handles client connections, WebSocket communication, and message broadcasting |
| **PostgreSQL** | Stores all chat events persistently — every create/join/leave/message is logged |
| **Redis** | Acts as a pub/sub message bus between server instances, keeping all nodes in sync |
| **RedisInsight** | Optional GUI for monitoring Redis channels and pub/sub events in real time |
| **Nginx** | Load balancer that distributes incoming WebSocket connections across server instances |
| **Clients (wscat / Browser)** | Connect via WebSocket, send commands, and receive real-time messages |

```
+------------------+        +------------------+
|    Client 1      |        |    Client 2      |
| (wscat/browser)  |        | (wscat/browser)  |
+--------+---------+        +--------+---------+
         |                           |
         |     WebSocket (ws://)     |
         v                           v
+------------------+        +------------------+
| Rust WS Server 1 |        | Rust WS Server 2 |
+--------+---------+        +--------+---------+
         |                           |
         +------- Redis Pub/Sub -----+
                       |
                       v
              +----------------+
              |     Redis      |
              +----------------+
                       |
                       v
              +----------------+
              |   PostgreSQL   |
              | (event log)    |
              +----------------+
```

---

## Event Flow

1. **Client connects** via WebSocket and sends a JSON command (create, join, leave, or message).

2. **Server handles the command** — updates in-memory `ChatState` for the room and broadcasts to connected clients in that room.

3. **Event is logged to PostgreSQL** — every event is inserted into the `events` table. New instances can replay missed events on startup.

4. **Event is published to Redis** — all other server instances receive the event via the `chat:events` pub/sub channel, ensuring real-time sync across nodes.

5. **Other instances broadcast** the event to their own local clients, completing the delivery.

6. *(Optional)* **RedisInsight** can subscribe to `chat:events` to monitor events in real time.

---

## Project Structure

```
.
├── src/                  # Rust source code
├── migrations/           # SQLx database migrations
├── nginx/
│   └── nginx.conf        # Nginx load balancer config
├── data/                 # Persistent data (if any)
├── Dockerfile
├── docker-compose.yml    # Orchestrates ws1, ws2, postgres, redis, nginx
├── Cargo.toml
├── .env                  # Production environment variables
└── .env.local            # Local development environment variables
```
