CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY,
    room TEXT NOT NULL,
    user_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);
