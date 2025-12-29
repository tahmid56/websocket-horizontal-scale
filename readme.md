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

<h1>System Architecture</h1>
This chat application is designed to be scalable, reliable, and multi-instance, with proper separation of concerns and persistent event logging.
| Component                             | Purpose                                                                                                    |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| **Rust WebSocket server (Actix Web)** | Handles client connections, WebSocket communication, and message broadcasting.                             |
| **PostgreSQL**                        | Stores all chat events persistently. Every action (room creation, join/leave, message) is logged.          |
| **Redis**                             | Acts as a pub/sub message bus between multiple server instances, ensuring that all nodes are synchronized. |
| **RedisInsight**                      | Optional GUI for monitoring Redis channels and pub/sub events.                                             |
| **Clients (wscat / Browser / Web)**   | Connect to the WebSocket server, send commands, and receive real-time messages.                            |
| **Optional Nginx load balancer**      | Distributes incoming WebSocket connections across multiple server instances.                               |

+------------------+          +------------------+
|   Client 1       |          |   Client 2       |
|  (wscat/browser) |          |  (wscat/browser) |
+--------+---------+          +--------+---------+
         |                            |
         | WebSocket (ws://)          |
         v                            v
+------------------+        +------------------+
| Rust WS Server 1 |        | Rust WS Server 2 |
+------------------+        +------------------+
         |                            |
         | Redis Pub/Sub (chat:events) 
         v
+-----------------------------+
|          Redis              |
+-----------------------------+
         |
         v
+-----------------------------+
|        PostgreSQL           |
|  (persistent event log)     |
+-----------------------------+


# Flow of Events

Client connects via WebSocket

Sends JSON commands: create room, join room, leave room, message.

Server handles commands

Updates in-memory ChatState for the room and subscribed users.

Sends the message to connected clients in the room.

Log event to PostgreSQL

Every event (create/join/leave/message) is inserted into the events table.

Allows new instances to replay missed events if they start later.

Publish to Redis pub/sub

Ensures all other server instances receive the event.

Supports multi-node scaling without losing events.

Other instances receive events

Redis subscriber broadcasts the events to their local clients.

Ensures real-time synchronization across all nodes.

Optional RedisInsight monitoring

Subscribe to the chat:events channel to watch events in real-time.