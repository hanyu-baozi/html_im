# HTML-IM 项目使用指南

## 一、项目运行端口

- **后端服务**: `8080` 端口（Rust Actix-web）
- **前端访问**: `80` 端口（Nginx 代理）
- **数据库**: `3306` 端口（MySQL）

### 当前访问地址
```
http://120.79.164.201
```

---

## 二、免费注册域名

### 方案 1：Freenom（免费顶级域名）

1. **访问官网**: https://www.freenom.com/
2. **搜索域名**: 输入你想要的域名（如 `mychat.tk`）
3. **注册账号**: 填写邮箱和密码
4. **选择免费域名**: 选择 `.tk`, `.ml`, `.ga`, `.cf`, `.gq` 后缀
5. **配置 DNS**:
   - 登录 Freenom 后台
   - 进入 `Services > My Domains > Manage Domain`
   - 点击 `Management Tools > Nameservers`
   - 选择 `Use custom nameservers`
   - 添加你的服务器 IP 或 Cloudflare DNS

### 方案 2：Cloudflare（推荐）

1. **注册 Cloudflare**: https://www.cloudflare.com/
2. **添加站点**: 输入你已购买的域名
3. **修改 DNS 服务器**: 在域名注册商处修改为 Cloudflare 提供的 DNS
4. **添加 A 记录**:
   - 类型: `A`
   - 名称: `@` 或 `www`
   - 内容: `120.79.164.201`
   - 代理状态: `DNS only`（橙色云朵关闭）

### 方案 3：国内免费二级域名

- **花生壳**: https://hsk.oray.com/（提供免费二级域名）
- **natapp**: https://natapp.cn/（免费隧道 + 域名）

---

## 三、项目更新操作

### 1. 更新后端代码

```bash
# SSH 登录服务器
ssh root@120.79.164.201

# 进入项目目录
cd ~/projects/html-im

# 拉取最新代码
git pull origin main

# 重新编译后端
cd backend
cargo build --release

# 重启后端服务
systemctl restart html-im-backend

# 查看状态
systemctl status html-im-backend
```

### 2. 更新前端代码

```bash
# 进入前端目录
cd ~/projects/html-im/frontend

# 拉取最新代码
git pull origin main

# 安装依赖（如果有新依赖）
npm install --legacy-peer-deps

# 重新构建
npm run build

# 重启 Nginx（如果需要）
systemctl reload nginx
```

### 3. 数据库迁移（如果有新表）

```bash
cd ~/projects/html-im/backend/migrations

# 执行新的 SQL 文件
mysql -u root -p123456 html_im < new_migration.sql
```

### 4. 完整更新脚本

创建更新脚本 `update.sh`:

```bash
#!/bin/bash
echo "开始更新项目..."

cd ~/projects/html-im
git pull origin main

echo "编译后端..."
cd backend
cargo build --release

echo "构建前端..."
cd ../frontend
npm install --legacy-peer-deps
npm run build

echo "重启服务..."
systemctl restart html-im-backend
systemctl reload nginx

echo "更新完成！"
```

使用方式:
```bash
chmod +x update.sh
./update.sh
```

---

## 四、内网穿透（本地开发测试）

### 方案 1：Cloudflare Tunnel（推荐，免费）

1. **安装 cloudflared**:
```bash
# Ubuntu/Debian
wget https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb
sudo dpkg -i cloudflared-linux-amd64.deb
```

2. **登录 Cloudflare**:
```bash
cloudflared tunnel login
```

3. **创建隧道**:
```bash
cloudflared tunnel create html-im
```

4. **配置隧道** (`~/.cloudflared/config.yml`):
```yaml
tunnel: <your-tunnel-id>
credentials-file: /root/.cloudflared/<tunnel-id>.json

ingress:
  - hostname: chat.yourdomain.com
    service: http://localhost:80
  - service: http_status:404
```

5. **运行隧道**:
```bash
cloudflared tunnel run html-im
```

### 方案 2：frp（开源内网穿透）

**服务器端（有公网 IP）**:
```bash
# 下载 frp
wget https://github.com/fatedier/frp/releases/download/v0.52.0/frp_0.52.0_linux_amd64.tar.gz
tar -zxvf frp_0.52.0_linux_amd64.tar.gz
cd frp_0.52.0_linux_amd64

# 配置 frps.ini
cat > frps.ini << EOF
[common]
bind_port = 7000
token = your_secret_token
EOF

# 启动服务端
./frps -c frps.ini
```

**客户端（内网机器）**:
```bash
# 配置 frpc.ini
cat > frpc.ini << EOF
[common]
server_addr = 120.79.164.201
server_port = 7000
token = your_secret_token

[web]
type = http
local_port = 80
custom_domains = chat.yourdomain.com
EOF

# 启动客户端
./frpc -c frpc.ini
```

### 方案 3：ngrok（简单快速）

```bash
# 注册 ngrok: https://ngrok.com/
# 下载并安装
wget https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-linux-amd64.tgz
tar -zxvf ngrok-v3-stable-linux-amd64.tgz

# 认证
./ngrok authtoken <your-authtoken>

# 启动穿透
./ngrok http 80
```

---

## 五、服务器安全防护

### 1. 配置防火墙（UFW）

```bash
# 安装 UFW
apt install ufw -y

# 允许 SSH（重要！先设置再启用）
ufw allow 22/tcp

# 允许 HTTP/HTTPS
ufw allow 80/tcp
ufw allow 443/tcp

# 允许后端端口（仅本地访问）
ufw allow 8080/tcp

# 允许 MySQL（仅本地）
ufw allow 3306/tcp

# 启用防火墙
ufw enable

# 查看状态
ufw status
```

### 2. 修改 SSH 端口（防止暴力破解）

```bash
# 编辑 SSH 配置
vim /etc/ssh/sshd_config

# 修改端口（例如改为 2222）
Port 2222

# 禁止 root 登录（可选）
PermitRootLogin no

# 重启 SSH 服务
systemctl restart sshd

# 测试新端口（不要关闭当前会话！）
ssh -p 2222 root@120.79.164.201
```

### 3. 安装 Fail2ban（防暴力破解）

```bash
apt install fail2ban -y

# 配置
cat > /etc/fail2ban/jail.local << EOF
[sshd]
enabled = true
port = 22
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600
findtime = 600
EOF

# 启动服务
systemctl start fail2ban
systemctl enable fail2ban

# 查看状态
fail2ban-client status sshd
```

### 4. 配置 HTTPS（Let's Encrypt 免费证书）

```bash
# 安装 Certbot
apt install certbot python3-certbot-nginx -y

# 获取证书（需要先有域名）
certbot --nginx -d yourdomain.com -d www.yourdomain.com

# 自动续期测试
certbot renew --dry-run
```

### 5. 限制 API 访问频率

在 Nginx 配置中添加限流:

```nginx
# 在 http 块中添加
limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;

# 在 location /api/ 中添加
location /api/ {
    limit_req zone=api burst=20 nodelay;
    proxy_pass http://127.0.0.1:8080;
    # ... 其他配置
}
```

### 6. 数据库安全

```bash
# 禁止远程访问 MySQL
vim /etc/mysql/mysql.conf.d/mysqld.cnf

# 确保 bind-address 为 127.0.0.1
bind-address = 127.0.0.1

# 重启 MySQL
systemctl restart mysql

# 删除匿名用户
mysql -u root -p
DELETE FROM mysql.user WHERE User='';
FLUSH PRIVILEGES;
```

### 7. 定期备份

```bash
# 创建备份脚本
cat > /root/backup.sh << 'EOF'
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR=/root/backups

mkdir -p $BACKUP_DIR

# 备份数据库
mysqldump -u root -p123456 html_im > $BACKUP_DIR/html_im_$DATE.sql

# 备份前端
tar -czf $BACKUP_DIR/frontend_$DATE.tar.gz /root/projects/html-im/frontend/dist

# 保留最近 7 天的备份
find $BACKUP_DIR -name "*.sql" -mtime +7 -delete
find $BACKUP_DIR -name "*.tar.gz" -mtime +7 -delete

echo "备份完成: $DATE"
EOF

chmod +x /root/backup.sh

# 添加到定时任务（每天凌晨 2 点备份）
crontab -e
# 添加: 0 2 * * * /root/backup.sh
```

### 8. 监控服务器状态

```bash
# 安装 htop（进程监控）
apt install htop -y

# 安装 netdata（实时监控面板）
bash <(curl -Ss https://my-netdata.io/kickstart.sh)

# 访问: http://120.79.164.201:19999
```

### 9. 安全清单

- [x] 启用防火墙（UFW）
- [x] 修改 SSH 端口
- [x] 安装 Fail2ban
- [x] 配置 HTTPS
- [x] 限制 API 访问频率
- [x] 数据库仅本地访问
- [x] 定期备份数据
- [x] 禁用不必要的服务
- [x] 定期更新系统: `apt update && apt upgrade -y`

---

## 六、常用维护命令

```bash
# 查看后端日志
journalctl -u html-im-backend -f

# 查看 Nginx 日志
tail -f /var/log/nginx/access.log
tail -f /var/log/nginx/error.log

# 查看系统资源
htop
df -h
free -m

# 重启所有服务
systemctl restart html-im-backend
systemctl reload nginx
systemctl restart mysql

# 查看端口占用
netstat -tulpn | grep LISTEN
```

---

## 七、故障排查

### 后端无法启动
```bash
# 查看日志
journalctl -u html-im-backend -n 50

# 检查端口占用
lsof -i :8080

# 手动启动测试
cd ~/projects/html-im/backend
./target/release/html-im-backend
```

### 前端无法访问
```bash
# 检查 Nginx 配置
nginx -t

# 查看错误日志
tail -f /var/log/nginx/error.log

# 检查文件权限
ls -la /root/projects/html-im/frontend/dist/
```

### 数据库连接失败
```bash
# 检查 MySQL 状态
systemctl status mysql

# 测试连接
mysql -u html_im -p123456 html_im

# 查看数据库日志
tail -f /var/log/mysql/error.log
```

---

## 八、联系与支持

- 项目地址: https://github.com/hanyu-baozi/html_im
- 问题反馈: GitHub Issues
