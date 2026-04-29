# HTML-IM 零基础部署指南（Ubuntu 22.04）

> 📖 本教程专为零基础小白设计，每一步都有详细说明，使用国内镜像加速安装

---

## 📋 目录

- [准备工作](#准备工作)
- [第一步：连接服务器](#第一步连接服务器)
- [第二步：更换国内软件源](#第二步更换国内软件源)
- [第三步：安装 MySQL 数据库](#第三步安装-mysql-数据库)
- [第四步：安装 Rust 编程环境](#第四步安装-rust-编程环境)
- [第五步：安装 Node.js](#第五步安装-nodejs)
- [第六步：安装 Nginx](#第六步安装-nginx)
- [第七步：下载项目代码](#第七步下载项目代码)
- [第八步：配置数据库](#第八步配置数据库)
- [第九步：编译后端程序](#第九步编译后端程序)
- [第十步：构建前端页面](#第十步构建前端页面)
- [第十一步：配置 Nginx](#第十一步配置-nginx)
- [第十二步：设置开机自启](#第十二步设置开机自启)
- [第十三步：测试访问](#第十三步测试访问)
- [附录：内网穿透教程](#附录内网穿透教程)

---

## 准备工作

### 你需要准备什么？

1. **阿里云服务器**（Ubuntu 22.04 系统）
2. **SSH 连接工具**（推荐使用 PowerShell、Xshell 或 FinalShell）
3. **耐心的心态**（整个过程大约需要 30-60 分钟）

### 阿里云安全组设置

登录阿里云控制台 → 找到你的服务器 → 安全组 → 添加入方向规则：

| 端口范围 | 协议 | 说明 |
|---------|------|------|
| 22/22 | TCP | SSH 远程登录 |
| 80/80 | TCP | 网站访问 |
| 443/443 | TCP | HTTPS 访问（可选） |
| 8080/8080 | TCP | 后端 API（可选） |

---

## 第一步：连接服务器

### Windows 用户

按 `Win + R` 键，输入 `powershell`，回车打开 PowerShell，然后输入：

```bash
ssh root@你的服务器IP地址
```

例如：
```bash
ssh root@47.96.123.45
```

首次连接会提示是否继续，输入 `yes` 回车，然后输入服务器密码。

> 💡 **提示**：输入密码时屏幕上不会显示任何字符，这是正常的，输完直接回车即可。

---

## 第二步：更换国内软件源

### 为什么要换源？

默认的国外服务器下载速度很慢，换成国内镜像源可以大幅提升下载速度。

### 备份原来的源

```bash
cp /etc/apt/sources.list /etc/apt/sources.list.bak
```

### 替换为阿里云镜像源

```bash
cat > /etc/apt/sources.list << 'EOF'
deb http://mirrors.aliyun.com/ubuntu/ jammy main restricted universe multiverse
deb http://mirrors.aliyun.com/ubuntu/ jammy-security main restricted universe multiverse
deb http://mirrors.aliyun.com/ubuntu/ jammy-updates main restricted universe multiverse
deb http://mirrors.aliyun.com/ubuntu/ jammy-backports main restricted universe multiverse
EOF
```

### 更新软件包列表

```bash
apt update
```

看到类似 `Reading package lists... Done` 的提示就说明更新成功了。

---

## 第三步：安装 MySQL 数据库

### 安装 MySQL

```bash
apt install -y mysql-server
```

等待安装完成，看到 `Setting up mysql-server` 类似的提示就说明安装成功了。

### 启动 MySQL 服务

```bash
systemctl start mysql
systemctl enable mysql
```

### 检查 MySQL 是否运行

```bash
systemctl status mysql
```

看到绿色的 `active (running)` 就说明 MySQL 正在运行。按 `q` 键退出。

### 登录 MySQL

```bash
mysql -u root
```

### 创建数据库和用户

在 MySQL 提示符下（看到 `mysql>`），依次输入以下命令（每行输入后按回车）：

```sql
CREATE DATABASE html_im CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

CREATE USER 'html_im'@'localhost' IDENTIFIED BY 'HtmlIm@2024';

GRANT ALL PRIVILEGES ON html_im.* TO 'html_im'@'localhost';

FLUSH PRIVILEGES;

EXIT;
```

> 💡 **提示**：密码 `HtmlIm@2024` 可以改成你自己想要的密码，但一定要记住！

---

## 第四步：安装 Rust 编程环境

### 使用国内镜像安装 Rust

```bash
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

curl --proto '=https' --tlsv1.2 -sSf https://mirrors.ustc.edu.cn/misc/rustup-install.sh | sh
```

安装时会提示选择安装类型，输入 `1` 回车（选择默认安装）。

### 让环境变量生效

```bash
source "$HOME/.cargo/env"
```

### 配置 Cargo 使用国内镜像

```bash
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
EOF
```

### 验证安装成功

```bash
rustc --version
cargo --version
```

看到版本号就说明安装成功了。

---

## 第五步：安装 Node.js

### 使用 NodeSource 安装 Node.js 18

```bash
curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
apt install -y nodejs
```

### 配置 npm 使用淘宝镜像

```bash
npm config set registry https://registry.npmmirror.com
```

### 验证安装成功

```bash
node --version
npm --version
```

看到版本号就说明安装成功了。

---

## 第六步：安装 Nginx

### 安装 Nginx

```bash
apt install -y nginx
```

### 启动 Nginx

```bash
systemctl start nginx
systemctl enable nginx
```

### 检查 Nginx 是否运行

```bash
systemctl status nginx
```

看到绿色的 `active (running)` 就说明 Nginx 正在运行。按 `q` 键退出。

---

## 第七步：下载项目代码

### 创建项目目录

```bash
mkdir -p ~/projects/html-im
cd ~/projects/html-im
```

### 下载项目

```bash
apt install -y git
git clone https://github.com/hanyu-baozi/html_im.git .
```

### 切换到稳定版本

```bash
git checkout v0.1.1
```

---

## 第八步：配置数据库连接

### 编辑后端配置文件

```bash
cd ~/projects/html-im/backend
apt install -y vim
vim .env
```

按 `i` 键进入编辑模式，修改文件内容为：

```env
DATABASE_URL=mysql://html_im:HtmlIm@2024@localhost:3306/html_im
REDIS_URL=redis://127.0.0.1:6379
JWT_SECRET=my-super-secret-key-2024-html-im-project
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

> ⚠️ **重要**：
> - 如果之前修改过数据库密码，这里的 `HtmlIm@2024` 也要改成你的密码
> - `JWT_SECRET` 建议改成更复杂的字符串

修改完成后，按 `Esc` 键，然后输入 `:wq` 回车保存退出。

### 创建上传目录

```bash
mkdir -p ~/projects/html-im/backend/uploads
chmod 755 ~/projects/html-im/backend/uploads
```

---

## 第九步：编译后端程序

### 开始编译

```bash
cd ~/projects/html-im/backend
cargo build --release
```

> ⏰ **注意**：首次编译需要 10-30 分钟，请耐心等待！
> 
> 编译过程中会下载很多依赖包，这是正常的。
> 
> 如果下载速度慢，可以按 `Ctrl+C` 中断，检查上面的 Cargo 镜像配置是否正确。

### 编译成功的标志

看到类似这样的提示就说明编译成功了：

```
Finished release [optimized] target(s) in XXm XXs
```

### 验证编译成功

```bash
ls -la target/release/backend
```

能看到文件信息就说明编译成功了。

---

## 第十步：构建前端页面

### 安装前端依赖

```bash
cd ~/projects/html-im/frontend
npm install
```

等待安装完成，看到 `added XXX packages` 就说明成功了。

### 构建前端

```bash
npm run build
```

看到 `dist/` 目录创建成功就说明构建完成了。

### 验证构建成功

```bash
ls -la dist/
```

能看到 `index.html` 等文件就说明构建成功了。

---

## 第十一步：配置 Nginx

### 创建 Nginx 配置文件

```bash
vim /etc/nginx/sites-available/html-im
```

按 `i` 键进入编辑模式，粘贴以下内容：

```nginx
server {
    listen 80;
    server_name _;

    # 前端静态文件
    location / {
        root /root/projects/html-im/frontend/dist;
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
        alias /root/projects/html-im/backend/uploads/;
        expires 30d;
        add_header Cache-Control "public, immutable";
    }

    # 文件上传大小限制
    client_max_body_size 10M;
}
```

按 `Esc` 键，输入 `:wq` 回车保存退出。

### 启用配置

```bash
ln -s /etc/nginx/sites-available/html-im /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default
```

### 测试配置是否正确

```bash
nginx -t
```

看到 `syntax is ok` 和 `test is successful` 就说明配置正确。

### 重新加载 Nginx

```bash
systemctl reload nginx
```

---

## 第十二步：设置开机自启

### 创建后端服务文件

```bash
vim /etc/systemd/system/html-im-backend.service
```

按 `i` 键进入编辑模式，粘贴以下内容：

```ini
[Unit]
Description=HTML-IM Backend Service
After=network.target mysql.service

[Service]
Type=simple
User=root
WorkingDirectory=/root/projects/html-im/backend
ExecStart=/root/projects/html-im/backend/target/release/backend
Restart=on-failure
RestartSec=5
Environment=DATABASE_URL=mysql://html_im:HtmlIm@2024@localhost:3306/html_im
Environment=JWT_SECRET=my-super-secret-key-2024-html-im-project
Environment=SERVER_HOST=0.0.0.0
Environment=SERVER_PORT=8080

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

> ⚠️ **注意**：如果修改过数据库密码，这里的 `HtmlIm@2024` 也要改成你的密码。

按 `Esc` 键，输入 `:wq` 回车保存退出。

### 启动服务

```bash
systemctl daemon-reload
systemctl start html-im-backend
systemctl enable html-im-backend
```

### 检查服务状态

```bash
systemctl status html-im-backend
```

看到绿色的 `active (running)` 就说明后端服务运行正常。按 `q` 键退出。

---

## 第十三步：测试访问

### 测试后端 API

```bash
curl http://localhost:8080/api/users
```

如果返回 JSON 数据，说明后端运行正常。

### 浏览器访问

打开浏览器，访问：

```
http://你的服务器IP地址
```

例如：`http://47.96.123.45`

如果能看到登录页面，说明部署成功！🎉

---

## 常见问题排查

### 问题 1：后端服务启动失败

```bash
# 查看详细错误日志
journalctl -u html-im-backend -n 50 --no-pager
```

常见原因：
- 数据库密码配置错误
- 数据库没有创建
- 端口被占用

### 问题 2：浏览器访问显示 502 错误

```bash
# 检查后端是否运行
curl http://127.0.0.1:8080/api/users

# 检查 Nginx 错误日志
tail -f /var/log/nginx/error.log
```

### 问题 3：WebSocket 连接失败

- 检查 Nginx 配置中是否包含 WebSocket 代理配置
- 检查阿里云安全组是否开放 80 端口
- 打开浏览器开发者工具（F12），查看 Console 中的错误信息

### 问题 4：上传文件失败

```bash
# 检查上传目录权限
ls -la ~/projects/html-im/backend/uploads/

# 修改权限
chmod 755 ~/projects/html-im/backend/uploads
```

### 问题 5：编译速度太慢

- 检查是否配置了 Cargo 国内镜像（见第四步）
- 检查网络连接是否正常
- 可以等待更长时间，首次编译确实很慢

---

## 如何更新项目？

当有新版本发布时，按以下步骤更新：

```bash
# 进入项目目录
cd ~/projects/html-im

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

## 附录：内网穿透教程

> 💡 **适用场景**：如果你没有云服务器，想在自己电脑上运行并让外网访问

### 方案一：使用 cpolar（推荐小白）

#### 1. 注册账号

访问 https://www.cpolar.com/ 注册账号

#### 2. 安装 cpolar

```bash
curl https://www.cpolar.com/static/downloads/install-release-cpolar.sh | sudo bash
```

#### 3. 登录 cpolar

```bash
cpolar authtoken 你的认证token
```

认证 token 在 cpolar 官网的控制台可以找到。

#### 4. 创建隧道

登录 cpolar 官网控制台 → 隧道管理 → 创建隧道：

- **隧道名称**：html-im
- **协议**：http
- **本地地址**：80
- **域名类型**：免费随机域名

#### 5. 启动 cpolar

```bash
cpolar start html-im
```

启动后会显示一个外网访问地址，例如：

```
https://abc123.cpolar.cn
```

把这个地址发给朋友，他们就能访问你的项目了！

### 方案二：使用 frp（适合有技术基础）

#### 1. 准备一台有公网 IP 的服务器

作为 frp 服务端（frps）。

#### 2. 在公网服务器上安装 frps

```bash
# 下载 frp
wget https://github.com/fatedier/frp/releases/download/v0.52.0/frp_0.52.0_linux_amd64.tar.gz

# 解压
tar -zxvf frp_0.52.0_linux_amd64.tar.gz
cd frp_0.52.0_linux_amd64

# 编辑配置文件
vim frps.ini
```

写入：

```ini
[common]
bind_port = 7000
```

启动 frps：

```bash
./frps -c frps.ini
```

#### 3. 在本地电脑上安装 frpc

下载 Windows 版 frp：
https://github.com/fatedier/frp/releases/download/v0.52.0/frp_0.52.0_windows_amd64.zip

解压后编辑 `frpc.ini`：

```ini
[common]
server_addr = 你的公网服务器IP
server_port = 7000

[web]
type = http
local_port = 80
custom_domains = 你的域名
```

启动 frpc：

```cmd
frpc.exe -c frpc.ini
```

### 方案三：使用 Cloudflare Tunnel（免费且稳定）

#### 1. 注册 Cloudflare 账号

访问 https://dash.cloudflare.com/ 注册

#### 2. 安装 cloudflared

```bash
# Ubuntu/Debian
curl -fsSL https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64 -o /usr/local/bin/cloudflared
chmod +x /usr/local/bin/cloudflared
```

#### 3. 登录并创建隧道

```bash
cloudflared tunnel login
```

会打开浏览器，选择你的域名进行授权。

#### 4. 创建隧道

```bash
cloudflared tunnel create html-im
```

#### 5. 配置隧道

```bash
vim ~/.cloudflared/config.yml
```

写入：

```yaml
tunnel: 你的tunnel-ID
credentials-file: /root/.cloudflared/你的tunnel-ID.json

ingress:
  - hostname: im.你的域名.com
    service: http://localhost:80
  - service: http_status:404
```

#### 6. 启动隧道

```bash
cloudflared tunnel run html-im
```

现在访问 `im.你的域名.com` 就能访问你的项目了！

---

## 内网穿透方案对比

| 方案 | 难度 | 费用 | 稳定性 | 推荐指数 |
|------|------|------|--------|---------|
| cpolar | ⭐ | 免费/付费 | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| frp | ⭐⭐⭐ | 需服务器 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| Cloudflare Tunnel | ⭐⭐ | 免费 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

---

## 总结

恭喜你完成了整个部署流程！🎉

如果遇到问题，可以：
1. 查看本文档的"常见问题排查"部分
2. 查看服务日志找错误信息
3. 在 GitHub 提交 Issue 提问

**项目地址**：https://github.com/hanyu-baozi/html_im
