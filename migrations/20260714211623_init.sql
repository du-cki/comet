CREATE TABLE IF NOT EXISTS users (
    id         INTEGER PRIMARY KEY,
    name       TEXT NOT NULL,
    email      TEXT UNIQUE NOT NULL,
    password   TEXT NOT NULL,
    api_key    TEXT UNIQUE NOT NULL,
    role       INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS settings (
    id                   INTEGER PRIMARY KEY CHECK (id = 1),

    allow_public_signups BOOLEAN NOT NULL DEFAULT 1,
    require_2fa          BOOLEAN NOT NULL DEFAULT 0,
    maintenance_mode     BOOLEAN NOT NULL DEFAULT 0,    
    max_upload_size_mb   INTEGER DEFAULT NULL
);

INSERT OR IGNORE INTO settings (id)
VALUES (1);

CREATE TABLE IF NOT EXISTS media (
    media_id           TEXT NOT NULL,
    file_path          TEXT NOT NULL,

    user_id            INTEGER NOT NULL,

    uploaded_at        INTEGER NOT NULL,
    content_type       TEXT NOT NULL,
    file_hash          TEXT NOT NULL,
    file_ext           TEXT,
    original_file_name TEXT,

    PRIMARY KEY(file_path, media_id)
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_users_email   ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_api_key ON users(api_key);
CREATE INDEX IF NOT EXISTS idx_media_user_id ON media(user_id);
