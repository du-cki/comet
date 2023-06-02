CREATE TABLE IF NOT EXISTS media (
    uploaded_at     INTEGER NOT NULL,
    media_id           TEXT NOT NULL,
    file_path          TEXT NOT NULL,
    content_type       TEXT NOT NULL,
    file_hash          TEXT NOT NULL,
    file_ext           TEXT,
    original_file_name TEXT,

    PRIMARY KEY(file_path, media_id)
);
