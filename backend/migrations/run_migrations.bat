@echo off
REM MySQL Database Setup Script for HTML-IM Backend (Windows)

set DB_HOST=localhost
set DB_PORT=3306
set DB_USER=root
set DB_PASSWORD=password
set DB_NAME=html_im

echo Setting up MySQL database for HTML-IM...

REM Create database
mysql -h %DB_HOST% -P %DB_PORT% -u %DB_USER% -p%DB_PASSWORD% -e "CREATE DATABASE IF NOT EXISTS %DB_NAME% CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
echo Database created: %DB_NAME%

REM Run migrations
mysql -h %DB_HOST% -P %DB_PORT% -u %DB_USER% -p%DB_PASSWORD% %DB_NAME% < create_users.sql
echo Created users table

mysql -h %DB_HOST% -P %DB_PORT% -u %DB_USER% -p%DB_PASSWORD% %DB_NAME% < create_messages.sql
echo Created messages table

mysql -h %DB_HOST% -P %DB_PORT% -u %DB_USER% -p%DB_PASSWORD% %DB_NAME% < create_sessions.sql
echo Created sessions table

echo Database setup completed!
pause
