# HTML-IM 服务器部署安全指南

## 部署前准备

### 1. 生成强密钥

```bash
# 生成 JWT Secret（64字节）
openssl rand -base64 64

# 生成数据库密码（32字节）
openssl rand -base64 32
```

### 2. 复制并配置环境变量

```bash
cd backend
cp .env.example .env
# 编辑 .env 文件，填入生成的密钥
```

### 3. 更新 .gitignore

确保 `.env` 文件已被 `.gitignore` 排除，避免敏感信息泄露。

---

## 服务器安全配置

### 1. 防火墙配置 (UFW)

```bash
# 安装 UFW
sudo apt install ufw

# 只开放必要端口
sudo ufw allow 22/tcp    # SSH（建议改为非标准端口）
sudo ufw allow 80/tcp    # HTTP（用于 Let's Encrypt 验证和重定向）
sudo ufw allow 443/tcp   # HTTPS

# 禁止 8080 和 3000 端口直接访问（仅允许本地）
# 后端和前端应该只绑定 127.0.0.1

# 启用防火墙
sudo ufw enable
sudo ufw status
```

### 2. SSH 安全加固

```bash
# 编辑 SSH 配置
sudo nano /etc/ssh/sshd_config

# 修改以下配置
Port 2222                    # 改为非标准端口
PermitRootLogin no           # 禁止 root 登录
PasswordAuthentication no    # 禁用密码登录，仅允许密钥
MaxAuthTries 3               # 最大认证尝试次数
ClientAliveInterval 300      # 空闲超时断开
ClientAliveCountMax 2

# 重启 SSH 服务
sudo systemctl restart sshd

# 配置 SSH 密钥登录
ssh-keygen -t ed25519 -C "your_email@example.com"
ssh-copy-id -p 2222 user@your-server-ip
```

### 3. 安装 Fail2Ban 防暴力破解

```bash
sudo apt install fail2ban

# 创建本地配置
sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local

# 编辑配置
sudo nano /etc/fail2ban/jail.local

# 添加以下配置
[sshd]
enabled = true
port = 2222
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600
findtime = 600

[html-im-backend]
enabled = true
port = 8080
filter = html-im
logpath = /var/log/html-im/*.log
maxretry = 10
bantime = 3600
findtime = 600

# 创建过滤器
sudo nano /etc/fail2ban/filter.d/html-im.conf

[Definition]
failregex = ^.*Failed login attempt.*IP: <HOST>.*$
            ^.*Invalid credentials.*IP: <HOST>.*$

# 启动服务
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
sudo fail2ban-client status
```

### 4. 安装并配置 Nginx 反向代理

```bash
sudo apt install nginx certbot python3-certbot-nginx

# 创建 Nginx 配置
sudo nano /etc/nginx/sites-available/html-im

server {
    listen 80;
    server_name your-domain.com www.your-domain.com;
    
    # 重定向到 HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name your-domain.com www.your-domain.com;
    
    # SSL 证书
    ssl_certificate /etc/letsencrypt/live/your-domain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/your-domain.com/privkey.pem;
    
    # SSL 配置
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;
    
    # 安全头
    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self' wss: https:;" always;
    
    # 前端静态文件（生产环境构建后）
    location / {
        root /var/www/html-im/frontend/dist;
        try_files $uri $uri/ /index.html;
        
        # 缓存静态资源
        location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
        }
    }
    
    # API 代理
    location /api/ {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # 限流配置
        limit_req zone=api burst=20 nodelay;
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
        
        # WebSocket 超时设置
        proxy_read_timeout 86400s;
        proxy_send_timeout 86400s;
    }
}

# 启用站点
sudo ln -s /etc/nginx/sites-available/html-im /etc/nginx/sites-enabled/

# 测试配置
sudo nginx -t

# 重启 Nginx
sudo systemctl restart nginx
```

### 5. 配置 Nginx 限流

```bash
# 编辑 Nginx 主配置
sudo nano /etc/nginx/nginx.conf

# 在 http 块中添加
http {
    # 限流区域定义
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_req_zone $binary_remote_addr zone=login:10m rate=5r/m;
    
    # 连接数限制
    limit_conn_zone $binary_remote_addr zone=addr:10m;
    
    # ... 其他配置
}
```

### 6. 申请 SSL 证书

```bash
# 申请 Let's Encrypt 证书
sudo certbot --nginx -d your-domain.com -d www.your-domain.com

# 自动续期
sudo certbot renew --dry-run

# 设置自动续期定时任务
sudo crontab -e
# 添加以下行
0 3 * * * certbot renew --quiet
```

---

## 数据库安全

### 1. MySQL 安全配置

```sql
-- 创建专用数据库用户
CREATE USER 'html_im_user'@'localhost' IDENTIFIED BY 'YOUR_STRONG_PASSWORD';
GRANT SELECT, INSERT, UPDATE, DELETE ON html_im.* TO 'html_im_user'@'localhost';
FLUSH PRIVILEGES;

-- 禁用远程 root 登录
UPDATE mysql.user SET Host='localhost' WHERE User='root' AND Host='%' ;
FLUSH PRIVILEGES;

-- 删除匿名用户
DELETE FROM mysql.user WHERE User='';
FLUSH PRIVILEGES;

-- 删除测试数据库
DROP DATABASE IF EXISTS test;
```

### 2. MySQL 配置加固

```bash
sudo nano /etc/mysql/mysql.conf.d/mysqld.cnf

# 添加以下配置
[mysqld]
bind-address = 127.0.0.1        # 只允许本地连接
skip-networking                 # 禁用 TCP/IP（如果不需要远程连接）
local-infile = 0                # 禁用 LOAD DATA LOCAL INFILE
secure-file-priv = ""           # 限制文件导入导出
max_connections = 100           # 限制最大连接数
```

### 3. 数据库备份

```bash
# 创建备份目录
sudo mkdir -p /backup/html-im
sudo chmod 700 /backup/html-im

# 创建备份脚本
sudo nano /usr/local/bin/backup-html-im.sh

#!/bin/bash
BACKUP_DIR="/backup/html-im"
DATE=$(date +%Y%m%d_%H%M%S)
DB_USER="html_im_user"
DB_PASS="YOUR_STRONG_PASSWORD"
DB_NAME="html_im"

# 创建备份
mysqldump -u $DB_USER -p$DB_PASS $DB_NAME | gzip > $BACKUP_DIR/html_im_$DATE.sql.gz

# 删除 30 天前的备份
find $BACKUP_DIR -name "html_im_*.sql.gz" -mtime +30 -delete

# 设置备份文件权限
chmod 600 $BACKUP_DIR/html_im_$DATE.sql.gz

# 设置脚本权限
sudo chmod 700 /usr/local/bin/backup-html-im.sh

# 添加定时任务
sudo crontab -e
# 每天凌晨 2 点备份
0 2 * * * /usr/local/bin/backup-html-im.sh
```

---

## 后端安全配置

### 1. 环境变量配置

```bash
# 后端 .env 文件
DATABASE_URL=mysql://html_im_user:STRONG_PASSWORD@localhost:3306/html_im
JWT_SECRET=使用 openssl rand -base64 64 生成的密钥
SERVER_HOST=127.0.0.1    # 只绑定本地
SERVER_PORT=8080
ALLOWED_ORIGIN=https://your-domain.com    # 限制 CORS 来源
RUST_LOG=warn           # 生产环境使用 warn 级别
```

### 2. 创建 Systemd 服务

```bash
sudo nano /etc/systemd/system/html-im-backend.service

[Unit]
Description=HTML-IM Backend Service
After=network.target mysql.service

[Service]
Type=simple
User=www-data
WorkingDirectory=/var/www/html-im/backend
ExecStart=/var/www/html-im/backend/target/release/html-im-backend
Restart=on-failure
RestartSec=5
EnvironmentFile=/var/www/html-im/backend/.env

# 安全限制
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/www/html-im/backend
PrivateTmp=true

[Install]
WantedBy=multi-user.target

# 启用并启动服务
sudo systemctl daemon-reload
sudo systemctl enable html-im-backend
sudo systemctl start html-im-backend
sudo systemctl status html-im-backend
```

---

## 前端安全配置

### 1. 生产环境构建

```bash
cd frontend

# 安装依赖
npm install

# 构建生产版本
npm run build

# 将构建产物部署到 Nginx
sudo cp -r dist/* /var/www/html-im/frontend/dist/
sudo chown -R www-data:www-data /var/www/html-im/frontend/dist
```

### 2. 前端 API 配置

确保前端使用 HTTPS 连接后端 API。

---

## 监控和日志

### 1. 配置日志轮转

```bash
sudo nano /etc/logrotate.d/html-im

/var/log/html-im/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 0640 www-data adm
    sharedscripts
    postrotate
        systemctl reload html-im-backend > /dev/null 2>&1 || true
    endscript
}
```

### 2. 安装监控工具

```bash
# 安装 htop
sudo apt install htop

# 安装 netdata（系统监控）
bash <(curl -Ss https://my-netdata.io/kickstart.sh)

# 查看日志
tail -f /var/log/html-im/backend.log
journalctl -u html-im-backend -f
```

---

## 安全扫描

### 1. 依赖安全扫描

```bash
# 前端
cd frontend
npm audit
npm audit fix

# 后端
cd backend
cargo audit      # 需要先安装: cargo install cargo-audit
```

### 2. 端口扫描检查

```bash
# 从外部扫描服务器
nmap -sV your-server-ip

# 应该只看到 22, 80, 443 端口开放
```

### 3. SSL 配置检查

```bash
# 使用 SSL Labs 检查
https://www.ssllabs.com/ssltest/analyze.html?d=your-domain.com
```

---

## 部署清单

### 部署前

- [ ] 生成强 JWT Secret 和数据库密码
- [ ] 配置 .env 文件
- [ ] 确认 .env 已添加到 .gitignore
- [ ] 修改 SSH 端口并禁用密码登录
- [ ] 配置防火墙规则
- [ ] 安装 Fail2Ban
- [ ] 配置 Nginx 反向代理
- [ ] 申请并配置 SSL 证书
- [ ] 配置数据库专用用户
- [ ] 设置数据库备份

### 部署后

- [ ] 测试 HTTPS 访问
- [ ] 测试 WebSocket 连接
- [ ] 验证 CORS 配置
- [ ] 检查端口开放情况
- [ ] 测试限流功能
- [ ] 验证备份脚本
- [ ] 配置监控告警
- [ ] 记录服务器信息和密码

---

## 应急响应

### 发现入侵迹象时

```bash
# 1. 查看登录记录
last
lastb
cat /var/log/auth.log

# 2. 查看异常进程
ps aux
htop

# 3. 查看网络连接
netstat -tulpn
ss -tulpn

# 4. 查看系统日志
journalctl -xe

# 5. 如果发现入侵，立即：
#    - 断开网络连接
#    - 保留日志证据
#    - 重置所有密码
#    - 审查访问记录
```

### 数据库被入侵时

```bash
# 1. 立即停止服务
sudo systemctl stop html-im-backend

# 2. 修改数据库密码
mysql -u root -p
ALTER USER 'html_im_user'@'localhost' IDENTIFIED BY 'NEW_STRONG_PASSWORD';
FLUSH PRIVILEGES;

# 3. 检查数据完整性
# 4. 恢复备份
# 5. 更新 .env 文件
# 6. 重启服务
```

---

## 定期维护

1. **每周**：检查系统日志和错误报告
2. **每月**：更新系统包和应用依赖
3. **每季度**：审查安全配置和防火墙规则
4. **每年**：轮换所有密钥和密码
