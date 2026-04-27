# MySQL 和 Redis 配置指南

## MySQL 数据库配置

### 1. 安装 MySQL

#### Windows
1. 下载 MySQL Community Server: https://dev.mysql.com/downloads/mysql/
2. 安装并设置 root 密码（默认: `password`）
3. 确保 MySQL 服务正在运行

#### macOS
```bash
brew install mysql
brew services start mysql
mysql_secure_installation
```

#### Linux (Ubuntu)
```bash
sudo apt update
sudo apt install mysql-server
sudo systemctl start mysql
sudo mysql_secure_installation
```

### 2. 创建数据库

#### 方法 1: 使用迁移脚本

**Windows:**
```cmd
cd backend\migrations
run_migrations.bat
```

**Linux/macOS:**
```bash
cd backend/migrations
chmod +x run_migrations.sh
./run_migrations.sh
```

#### 方法 2: 手动执行 SQL

```bash
mysql -u root -p
```

然后在 MySQL 命令行中执行：
```sql
CREATE DATABASE IF NOT EXISTS html_im CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE html_im;

-- 执行 migrations 目录下的 SQL 文件
SOURCE migrations/create_users.sql;
SOURCE migrations/create_messages.sql;
SOURCE migrations/create_sessions.sql;
```

### 3. 验证数据库

```sql
USE html_im;
SHOW TABLES;
```

应该看到三个表：`users`, `messages`, `sessions`

## Redis 配置

### 1. 安装 Redis

#### Windows
1. 下载 Redis for Windows: https://github.com/microsoftarchive/redis/releases
2. 解压并运行 `redis-server.exe`

#### macOS
```bash
brew install redis
brew services start redis
```

#### Linux (Ubuntu)
```bash
sudo apt update
sudo apt install redis-server
sudo systemctl start redis
sudo systemctl enable redis
```

### 2. 测试 Redis 连接

```bash
redis-cli ping
```

应该返回 `PONG`

### 3. 配置 Redis 密码（可选）

编辑 Redis 配置文件 `redis.conf`：
```
requirepass your_redis_password
```

重启 Redis 服务。

## 环境变量配置

更新 `backend/.env` 文件：

```env
# MySQL 配置
DATABASE_URL=mysql://root:password@localhost:3306/html_im

# Redis 配置
REDIS_URL=redis://127.0.0.1:6379
# 如果 Redis 有密码：
# REDIS_URL=redis://:your_redis_password@127.0.0.1:6379

# JWT 密钥（生产环境请修改）
JWT_SECRET=your-secret-key-change-in-production

# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

## 验证配置

### 测试 MySQL 连接
```bash
mysql -u root -p -e "SELECT 1"
```

### 测试 Redis 连接
```bash
redis-cli ping
```

### 启动后端服务
```bash
cd backend
cargo run
```

如果一切正常，应该看到：
```
[INFO] Starting HTML-IM Backend Server...
[INFO] Connecting to database: mysql://root:password@localhost:3306/html_im
[INFO] Database connection established
```

## 常见问题

### MySQL 连接失败
- 检查 MySQL 服务是否运行
- 验证用户名和密码
- 检查防火墙设置

### Redis 连接失败
- 检查 Redis 服务是否运行
- 验证 Redis 端口（默认 6379）
- 检查 Redis 密码配置

### 数据库表不存在
- 确保已执行迁移脚本
- 检查数据库名称是否正确
