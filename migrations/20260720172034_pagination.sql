CREATE INDEX IF NOT EXISTS idx_media_user_pagination 
ON media(user_id, uploaded_at DESC, media_id DESC);
