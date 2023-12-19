-- folder impl
CREATE TABLE IF NOT EXISTS folders (
    folder_id INTEGER PRIMARY KEY AUTOINCREMENT,
    folder_name TEXT NOT NULL,
    parent_folder_id INTEGER,
    FOREIGN KEY (parent_folder_id) REFERENCES folders(folder_id)
);

-- Add the folder_id column to the media table
ALTER TABLE media
ADD COLUMN folder_id INTEGER;

-- a ""boolean"" that indicates if the file is hidden on the public dashboard or not.
ALTER TABLE media ADD file_is_public BOOLEAN NOT NULL
      CHECK (file_is_public IN (0, 1)) DEFAULT 1;
