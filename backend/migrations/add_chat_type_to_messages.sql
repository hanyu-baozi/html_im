-- Add chat_type column to messages table
ALTER TABLE messages ADD COLUMN chat_type VARCHAR(20) NOT NULL DEFAULT 'private';

-- Drop the foreign key constraint on receiver_id since it can now be a group ID
ALTER TABLE messages DROP FOREIGN KEY messages_ibfk_2;

-- Add index on chat_type
CREATE INDEX idx_chat_type ON messages(chat_type);
