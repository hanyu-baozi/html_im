-- Create database
CREATE DATABASE IF NOT EXISTS html_im 
CHARACTER SET utf8mb4 
COLLATE utf8mb4_unicode_ci;

USE html_im;

-- Source migration files
SOURCE migrations/create_users.sql;
SOURCE migrations/create_messages.sql;
SOURCE migrations/create_sessions.sql;
