# HTML-IM 部署指南（Ubuntu 22.04）

> 本指南详细说明如何将 HTML-IM 项目从零部署到阿里云 Ubuntu 22.04 服务器

## 前置准备

### 1. 服务器要求
- 操作系统：Ubuntu 22.04 LTS
- 内存：建议 2GB 以上
- 磁盘：建议 20GB 以上
- 网络：已配置公网 IP（阿里云深圳节点）

### 2. 域名准备（可选）
- 如果有域名，需提前解析到服务器 IP
- 如果使用 IP 直接访问，可跳过此步骤

### 3. 阿里云安全组配置
在阿里云控制台开放以下端口：
| 端口 | 协议 | 用途 |
|------|------|------|
| 22 | TCP | SSH 远程连接 |
| 80 | TCP | HTTP 访问 |
| 443 | TCP | HTTPS 访问（可选） |
| 8080 | TCP | 后端 API（如直接访问） |

---

## 第一步：连接服务器

```bash
# Windows 用户使用 PowerShell 或 SSH 客户端
ssh root@你的服务器IP

# 例如：
ssh root@47.96.xxx.xxx
```

---

## 第二步：更新系统并安装基础依赖

```bash
# 更新系统包
apt update && apt upgrade -y

# 安装基础工具
apt install -y curl wget git vim build-essential pkg-config libssl-dev libmysqlclient-dev
```

---

## 第三步：安装 MySQL 数据库

```bash
# 安装 MySQL 8.0
apt install -y mysql-server

# 启动 MySQL 服务
systemctl start mysql
systemctl enable mysql

# 安全初始化（可选）
mysql_secure_installation

# 登录 MySQL
mysql -u root -p
```

在 MySQL 中执行以下命令：

```sql
-- 创建数据库
CREATE DATABASE html_im CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 创建专用用户（推荐，替换 your_password 为强密码）
CREATE USER 'html_im'@'localhost' IDENTIFIED BY 'your_password';

-- 授权
GRANT ALL PRIVILEGES ON html_im.* TO 'html_im'@'localhost';
FLUSH PRIVILEGES;

-- 退出
EXIT;
```

---

## 第四步：安装 Rust 环境

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 选择默认安装（选项 1）
# 安装完成后重新加载环境变量
source "$HOME/.cargo/env"

# 验证安装
rustc --version
cargo --version
```

---

## 第五步：安装 Node.js 和 npm

```bash
# 安装 Node.js 18.x
curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
apt install -y nodejs

# 验证安装
node --version
npm --version
```

---

## 第六步：克隆项目代码

```bash
# 创建项目目录
mkdir -p /opt/html-im
cd /opt/html-im

# 克隆项目（使用 SSH 或 HTTPS）
git clone git@github.com:hanyu-baozi/html_im.git .

# 如果使用 HTTPS：
# git clone https://github.com/hanyu-baozi/html_im.git .

# 切换到最新稳定版本
git checkout v0.1.1
```

---

## 第七步：配置后端

### 1. 修改环境变量

```bash
cd /opt/html-im/backend
vim .env
```

修改 `.env` 文件内容：

```env
# 数据库连接（使用刚才创建的专用用户）
DATABASE_URL=mysql://html_im:your_password@localhost:3306/html_im

# Redis（如果未安装可暂时不使用）
REDIS_URL=redis://127.0.0.1:6379

# JWT 密钥（务必修改为强随机字符串）
JWT_SECRET=your-super-secret-key-change-this-in-production-2024

# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

### 2. 创建上传目录

```bash
mkdir -p /opt/html-im/backend/uploads
chmod 755 /opt/html-im/backend/uploads
```

### 3. 编译后端

```bash
cd /opt/html-im/backend

# 编译 Release 版本（优化性能）
cargo build --release

# 编译完成后，可执行文件位于：
# /opt/html-im/backend/target/release/backend
```

> 注意：首次编译可能需要 10-20 分钟，请耐心等待。

---

## 第八步：构建前端

```bash
cd /opt/html-im/frontend

# 安装依赖
npm install

# 构建生产版本
npm run build

# 构建完成后，静态文件位于：
# /opt/html-im/frontend/dist/
```

---

## 第九步：安装配置 Nginx

```bash
# 安装 Nginx
apt install -y nginx

# 启动 Nginx
systemctl start nginx
systemctl enable nginx
```

### 创建 Nginx 配置文件

```bash
vim /etc/nginx/sites-available/html-im
```

写入以下内容：

```nginx
server {
    listen 80;
    server_name 你的域名或IP;  # 例如：47.96.xxx.xxx 或 im.example.com

    # 前端静态文件
    location / {
        root /opt/html-im/frontend/dist;
        try_files $uri $uri/ /index.html;
        index index.html;
    }

    # 后端 API 代理
    location /api/ {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_read_timeout 86400;
    }

    # WebSocket 代理
    location /ws {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_read_timeout 86400;
    }

    # 上传文件访问
    location /uploads/ {
        alias /opt/html-im/backend/uploads/;
        expires 30d;
        add_header Cache-Control "public, immutable";
    }

    # 文件上传大小限制
    client_max_body_size 10M;
}
```

### 启用配置

```bash
# 创建软链接
ln -s /etc/nginx/sites-available/html-im /etc/nginx/sites-enabled/

# 删除默认配置（可选）
rm -f /etc/nginx/sites-enabled/default

# 测试配置
nginx -t

# 重新加载 Nginx
systemctl reload nginx
```

---

## 第十步：创建 systemd 服务（后端）

```bash
vim /etc/systemd/system/html-im-backend.service
```

写入以下内容：

```ini
[Unit]
Description=HTML-IM Backend Service
After=network.target mysql.service

[Service]
Type=simple
User=root
WorkingDirectory=/opt/html-im/backend
ExecStart=/opt/html-im/backend/target/release/backend
Restart=on-failure
RestartSec=5
Environment=DATABASE_URL=mysql://html_im:your_password@localhost:3306/html_im
Environment=JWT_SECRET=your-super-secret-key-change-this-in-production-2024
Environment=SERVER_HOST=0.0.0.0
Environment=SERVER_PORT=8080

# 日志输出
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

> 注意：修改 `DATABASE_URL` 和 `JWT_SECRET` 为实际值。

### 启动服务

```bash
# 重新加载 systemd 配置
systemctl daemon-reload

# 启动后端服务
systemctl start html-im-backend

# 设置开机自启
systemctl enable html-im-backend

# 查看服务状态
systemctl status html-im-backend

# 查看日志
journalctl -u html-im-backend -f
```

---

## 第十一步：配置防火墙

```bash
# 如果使用 ufw 防火墙
apt install -y ufw

# 允许 SSH、HTTP、HTTPS
ufw allow 22/tcp
ufw allow 80/tcp
ufw allow 443/tcp

# 启用防火墙
ufw enable

# 查看状态
ufw status
```

---

## 第十二步：验证部署

### 1. 检查服务状态

```bash
# 检查后端服务
systemctl status html-im-backend

# 检查 Nginx
systemctl status nginx

# 检查 MySQL
systemctl status mysql
```

### 2. 测试 API

```bash
# 测试后端 API
curl http://localhost:8080/api/users

# 或通过 Nginx 访问
curl http://你的服务器IP/api/users
```

### 3. 浏览器访问

打开浏览器访问：
- `http://你的服务器IP` 或 `http://你的域名`

---

## 可选：配置 HTTPS（推荐）

### 使用 Let's Encrypt 免费证书

```bash
# 安装 certbot
apt install -y certbot python3-certbot-nginx

# 获取证书（替换为你的域名）
certbot --nginx -d im.example.com

# 自动续期
certbot renew --dry-run
```

---

## 常见问题排查

### 1. 后端启动失败

```bash
# 查看详细日志
journalctl -u html-im-backend -n 100 --no-pager

# 检查数据库连接
mysql -u html_im -p -e "SHOW DATABASES;"
```

### 2. Nginx 502 错误

```bash
# 检查后端是否运行
curl http://127.0.0.1:8080/api/users

# 检查 Nginx 错误日志
tail -f /var/log/nginx/error.log
```

### 3. WebSocket 连接失败

- 确保 Nginx 配置中包含 WebSocket 代理配置
- 检查安全组是否开放相应端口
- 查看浏览器控制台错误信息

### 4. 上传文件失败

```bash
# 检查上传目录权限
ls -la /opt/html-im/backend/uploads/

# 修改权限
chmod 755 /opt/html-im/backend/uploads
chown www-data:www-data /opt/html-im/backend/uploads
```

---

## 更新部署

当有新版本需要更新时：

```bash
cd /opt/html-im

# 拉取最新代码
git pull

# 切换到新版本
git checkout v0.2.0

# 重新编译后端
cd backend
cargo build --release

# 重启后端服务
systemctl restart html-im-backend

# 重新构建前端
cd ../frontend
npm install
npm run build

# 重新加载 Nginx
systemctl reload nginx
```

---

## 备份与恢复

### 数据库备份

```bash
# 备份数据库
mysqldump -u html_im -p html_im > /backup/html_im_$(date +%Y%m%d).sql

# 恢复数据库
mysql -u html_im -p html_im < /backup/html_im_20240101.sql
```

### 上传文件备份

```bash
# 备份上传文件
tar -czf /backup/uploads_$(date +%Y%m%d).tar.gz /opt/html-im/backend/uploads/
```

---

## 性能优化建议

### 1. MySQL 优化

编辑 `/etc/mysql/mysql.conf.d/mysqld.cnf`：

```ini
[mysqld]
max_connections = 200
innodb_buffer_pool_size = 512M
query_cache_size = 64M
```

### 2. Nginx 优化

在 `nginx.conf` 的 `http` 块中添加：

```nginx
gzip on;
gzip_types text/plain text/css application/json application/javascript text/xml;
gzip_min_length 1000;
```

### 3. 后端优化

- 使用 `cargo build --release` 编译（已包含）
- 考虑使用 Redis 缓存热点数据
- 配置日志轮转防止日志过大

---

## 监控与维护

### 查看服务日志

```bash
# 后端日志
journalctl -u html-im-backend -f

# Nginx 访问日志
tail -f /var/log/nginx/access.log

# Nginx 错误日志
tail -f /var/log/nginx/error.log

# MySQL 日志
tail -f /var/log/mysql/error.log
```

### 系统资源监控

```bash
# 查看 CPU 和内存使用
htop

# 查看磁盘使用
df -h

# 查看网络连接
netstat -tulpn | grep :8080
```
