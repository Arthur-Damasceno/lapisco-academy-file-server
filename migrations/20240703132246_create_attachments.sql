CREATE TABLE IF NOT EXISTS attachments (
    id TEXT PRIMARY KEY NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL
);
