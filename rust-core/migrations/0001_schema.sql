CREATE TABLE dblove_migrations (
    id INT NOT NULL, 
    name VARCHAR(255) NOT NULL,
    hash VARCHAR(64) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(id)
);

CREATE TABLE sync_folder(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    directory TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL,
    last_scan DATETIME,
    image_count INT NOT NULL,
    video_count INT NOT NULL,
    live_photo_count INT NOT NULL,
    ignored_count INT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE media(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sha1 BYTEA,
    ts DATETIME,
    remote_id TEXT UNIQUE,
    local_reference TEXT UNIQUE,
    local_size INT,
    key BYTEA,
    meta_data BYTEA,
    type INT NOT NULL,
    state INT NOT NULL,
    updated_at DATETIME,
    file_name TEXT NOT NULL DEFAULT '',
    sort_order INTEGER NOT NULL DEFAULT 0,
    error_count INTEGER NOT NULL DEFAULT 0,
    version INT NOT NULL DEFAULT 0,
    format SMALLINT NOT NULL DEFAULT 0,
    error_message TEXT,
    local_bucket TEXT NOT NULL DEFAULT '',
    liked INT NOT NULL DEFAULT 0,
    hidden INT NOT NULL DEFAULT 0,
    dirty INT NOT NULL DEFAULT 0
);

CREATE INDEX idx_sort_order ON media(sort_order);

CREATE TABLE cache_access_stat(
    type INTEGER NOT NULL,
    reference INTEGER NOT NULL,
    part INTEGER NOT NULL,
    size INTEGER NOT NULL DEFAULT 0,
    accessed_at INTEGER NOT NULL DEFAULT 0,
    extension TEXT NOT NULL DEFAULT '',
    PRIMARY KEY(type, reference)
);

CREATE TABLE log(
    ts INTEGER NOT NULL,
    level INTEGER NOT NULL,
    message TEXT NOT NULL,
    attributes TEXT NOT NULL
);

CREATE TABLE search_label(
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    language INTEGER NOT NULL,
    label_type INTEGER NOT NULL,
    text TEXT NOT NULL,
    autocomplete_order INETEGER NOT NULL, dirty SMALLINT NOT NULL DEFAULT 0, state SMALLINT NOT NULL DEFAULT 0, remote_id TEXT DEFAULT NULL,
    UNIQUE (language, label_type, text)
);

CREATE TABLE search_label_media(
    media_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,
    dirty SMALLINT NOT NULL DEFAULT 0,
    FOREIGN KEY(media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY(label_id) REFERENCES search_label(id),
    PRIMARY KEY (media_id, label_id)
);

CREATE INDEX idx_search_label ON search_label(text);

CREATE TABLE collection(
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    remote_id TEXT UNIQUE,
    cover_media_id INTEGER NULL,
    dirty INTEGER NOT NULL DEFAULT 0,
    updated_at DATETIME NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    state INT NOT NULL DEFAULT 0,
    key BYTEA, is_adhoc BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY(cover_media_id) REFERENCES media(id) ON DELETE SET NULL
);

CREATE TABLE collection_media(
    media_id INTEGER NOT NULL,
    collection_id INTEGER NOT NULL,
    remote_id TEXT DEFAULT NULL,
    dirty INT NOT NULL DEFAULT 0,
    state INT NOT NULL DEFAULT 0,
    updated_at DATETIME NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (collection_id) REFERENCES collection(id) ON DELETE CASCADE,
    PRIMARY KEY (media_id, collection_id) 
);

CREATE INDEX idx_search_label_remote_id ON search_label(remote_id);