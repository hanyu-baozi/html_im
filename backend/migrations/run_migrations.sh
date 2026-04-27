#!/bin/bash

# MySQL Database Setup Script for HTML-IM Backend

DB_HOST="localhost"
DB_PORT="3306"
DB_USER="root"
DB_PASSWORD="password"
DB_NAME="html_im"

echo "Setting up MySQL database for HTML-IM..."

# Create database
mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASSWORD" << EOF
CREATE DATABASE IF NOT EXISTS $DB_NAME CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
EOF

echo "Database created: $DB_NAME"

# Run migrations
mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASSWORD" "$DB_NAME" < create_users.sql
echo "Created users table"

mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASSWORD" "$DB_NAME" < create_messages.sql
echo "Created messages table"

mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASSWORD" "$DB_NAME" < create_sessions.sql
echo "Created sessions table"

echo "Database setup completed!"
