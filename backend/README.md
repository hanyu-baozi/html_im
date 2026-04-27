# HTML-IM Backend

基于 Rust (Actix-web) 的 IM 后端服务

## 技术栈

- **语言**: Rust 2021 Edition
- **Web 框架**: Actix-web 4.4
- **WebSocket**: Actix-ws 0.3
- **ORM**: SeaORM 0.12 (MySQL)
- **异步运行时**: Tokio 1.35
- **认证**: JWT (jsonwebtoken 9.2) + bcrypt 0.15
- **缓存**: Redis 0.24
- **序列化**: Serde + Serde_json

## 项目结构

```
backend/
├── src/
│   ├── main.rs           # 应用入口
│   ├── config.rs         # 配置管理
│   ├── db.rs             # 数据库连接
│   ├── models/           # 数据模型
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── message.rs
│   │   └── session.rs
│   ├── routes/           # 路由处理器
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── user.rs
│   │   └── message.rs
│   ├── websocket/        # WebSocket 处理
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   └── manager.rs
│   └── middleware/       # 中间件
│       ├── mod.rs
│       ├── auth.rs
│       └── logging.rs
├── Cargo.toml            # Rust 依赖配置
└── .env                  # 环境变量
```

## 环境配置

在 `.env` 文件中配置以下环境变量：

```env
DATABASE_URL=mysql://root:password@localhost:3306/html_im
REDIS_URL=redis://127.0.0.1:6379
JWT_SECRET=your-secret-key-change-in-production
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

## 依赖安装

```bash
cargo build
```

## 运行

```bash
cargo run
```

服务器将在 `http://0.0.0.0:8080` 启动

## API 端点

### 健康检查
- `GET /` - 服务健康状态

### 认证
- `POST /api/auth/register` - 用户注册
- `POST /api/auth/login` - 用户登录
- `POST /api/auth/logout` - 用户登出

### 用户
- `GET /api/users/me` - 获取当前用户信息
- `GET /api/users/{id}` - 获取指定用户信息

### 消息
- `GET /api/messages?contact_id=xxx` - 获取消息历史
- `POST /api/messages` - 发送消息
- `POST /api/messages/{id}/read` - 标记消息已读

### WebSocket
- `WS /ws` - WebSocket 实时通信

## 数据库表

### users
- id (varchar(36), PK)
- username (varchar(50))
- email (varchar(100))
- password_hash (varchar(255))
- avatar_url (varchar(255))
- status (varchar(20))
- created_at (datetime)
- updated_at (datetime)

### messages
- id (varchar(36), PK)
- sender_id (varchar(36), FK)
- receiver_id (varchar(36), FK)
- content (text)
- message_type (varchar(20))
- timestamp (datetime)
- is_read (tinyint(1))
- read_at (datetime)

### sessions
- id (varchar(36), PK)
- user1_id (varchar(36), FK)
- user2_id (varchar(36), FK)
- last_message_at (datetime)
- created_at (datetime)

## 开发说明

### TODO 事项

1. 实现用户注册/登录的数据库操作
2. 实现 JWT 认证中间件集成
3. 实现消息持久化到数据库
4. 实现 WebSocket 消息广播
5. 实现 Redis 缓存集成
6. 实现文件上传功能
7. 实现群组功能

### 数据库迁移

使用 SeaORM 迁移工具创建数据库表：

```bash
sea-orm-cli migrate generate create_users
sea-orm-cli migrate generate create_messages
sea-orm-cli migrate generate create_sessions
sea-orm-cli migrate up
```

## License

MIT
