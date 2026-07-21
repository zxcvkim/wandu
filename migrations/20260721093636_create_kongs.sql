CREATE TABLE IF NOT EXISTS kongs (
    id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    profile    TEXT    NOT NULL,
    content    TEXT    NOT NULL DEFAULT '',
    created_at TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TRIGGER IF NOT EXISTS update_kongs_updated_at
AFTER UPDATE ON kongs
FOR EACH ROW
WHEN NEW.updated_at IS OLD.updated_at
BEGIN
    UPDATE kongs
    SET updated_at = datetime('now')
    WHERE id = NEW.id;
END;
