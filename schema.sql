
CREATE TABLE IF NOT EXISTS media (
    file_name            TEXT       NOT NULL,
    is_public            BOOLEAN    NOT NULL CHECK (is_public IN (0, 1)) DEFAULT 0,
    uploaded_at          INTEGER    NOT NULL,
    last_updated_at      INTEGER,
    folder_id            INTEGER,
    original_file_name   TEXT,

    -- internal-use columns
    file_id              INTEGER    PRIMARY KEY AUTOINCREMENT,
    file_path            TEXT       NOT NULL,
    content_type         TEXT       NOT NULL,
    file_hash            TEXT       NOT NULL, -- storing the file has to save on space if duplicate files are uploaded.
    file_ext             TEXT,

    FOREIGN KEY (folder_id) REFERENCES folders(folder_id)
);

CREATE TABLE IF NOT EXISTS folders (
    parent_folder_id     INTEGER,
    folder_id            INTEGER    PRIMARY KEY AUTOINCREMENT,
    folder_name          TEXT       NOT NULL,

    is_public            BOOLEAN    NOT NULL CHECK (is_public IN (0, 1)) DEFAULT 0,

    FOREIGN KEY (parent_folder_id) REFERENCES folders(folder_id)
);


CREATE VIEW IF NOT EXISTS folder_paths AS WITH RECURSIVE tmp(id, path) AS (
   SELECT NULL, '/'
   UNION ALL

   SELECT folder_id, '/' || folder_name
   FROM folders
   WHERE parent_folder_id IS NULL

   UNION ALL

   SELECT
       folder_id,
       tmp.path || '/' || folders.folder_name AS NAME
   FROM folders
       JOIN tmp ON folders.parent_folder_id = tmp.id
) SELECT * FROM tmp;


-- database side checks

-- As much as I would like to make this trigger for both INSERT and UPDATE at a time,
-- SQLite doesn't support this. 
CREATE TRIGGER IF NOT EXISTS check_folder_name_exists_in_folder
BEFORE INSERT --, UPDATE
ON folders
BEGIN
    SELECT RAISE(FAIL, "name already exists within folder")
    FROM folders
    WHERE folder_name = NEW.folder_name
      AND parent_folder_id = NEW.parent_folder_id;
END;

CREATE TRIGGER IF NOT EXISTS update_last_updated
AFTER UPDATE
ON media
BEGIN
    UPDATE media
      SET last_updated_at = unixepoch()
          WHERE file_id = NEW.file_id;
END;
