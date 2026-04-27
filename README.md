# HTML-IM 即时通信系统

> 基于 Rust (Actix-web) + Vue 3 + WebSocket 的现代化即时通信应用

![版本](https://img.shields.io/badge/version-0.1.0-blue)
![许可证](https://img.shields.io/badge/license-MIT-green)

## 项目简介

HTML-IM 是一个全栈即时通信系统，支持好友私聊、群聊、消息实时推送等功能。采用现代化的技术栈构建，具有高性能、高可用的特点。

## 功能特性

- **用户认证**：支持注册、登录、Token 认证
- **好友管理**：支持按用户名/邮箱搜索好友、添加好友、删除好友
- **私聊功能**：支持一对一实时消息发送和接收
- **群聊功能**：支持创建群聊、添加群成员、群消息发送
- **实时通信**：基于 WebSocket 实现消息实时推送
- **在线状态**：实时显示用户在线/离线状态
- **消息历史**：支持查看历史消息记录
- **表情支持**：内置常用表情库

## 技术栈

### 前端
- **框架**: Vue 3 + Composition API
- **构建工具**: Vite 5.x
- **UI 样式**: Tailwind CSS
- **HTTP 客户端**: Axios
- **路由**: Vue Router

### 后端
- **语言**: Rust
- **Web 框架**: Actix-web 4.x
- **异步运行时**: Tokio
- **ORM**: SeaORM 0.12.x
- **数据库**: MySQL 8.0+
- **认证**: JWT (jsonwebtoken)
- **密码加密**: bcrypt
- **WebSocket**: actix-web-actors
- **序列化**: Serde + JSON

## 项目结构

```
html-im/
├── frontend/              # Vue 3 前端项目
│   ├── src/
│   │   ├── views/         # 页面组件
│   │   ├── services/      # API 服务
│   │   ├── router/        # 路由配置
│   │   └── App.vue        # 根组件
│   ├── package.json
│   └── vite.config.ts
├── backend/               # Rust 后端项目
│   ├── src/
│   │   ├── routes/        # API 路由
│   │   ├── models/        # 数据模型
│   │   ├── middleware/    # 中间件
│   │   ├── websocket/     # WebSocket 处理
│   │   ├── db.rs          # 数据库连接
│   │   └── main.rs        # 程序入口
│   ├── migrations/        # 数据库迁移
│   ├── Cargo.toml
│   └── .env
└── README.md
```

## API 接口

### 认证接口
| 方法 | 路径 | 描述 |
|------|------|------|
| POST | /api/auth/register | 用户注册 |
| POST | /api/auth/login | 用户登录 |
| POST | /api/auth/logout | 用户退出 |

### 用户接口
| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/users/me | 获取当前用户信息 |
| GET | /api/users | 获取所有用户列表 |
| GET | /api/users/search | 按用户名搜索用户 |
| GET | /api/users/search-by-email | 按邮箱搜索用户 |
| PUT | /api/users/status | 更新用户状态 |

### 好友接口
| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/friends | 获取好友列表 |
| POST | /api/friends/add | 添加好友 |
| POST | /api/friends/add-by-email | 按邮箱添加好友 |
| DELETE | /api/friends/remove | 删除好友 |

### 消息接口
| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/messages | 获取消息列表 |
| POST | /api/messages | 发送消息 |
| POST | /api/messages/:id/read | 标记消息为已读 |

### 群聊接口
| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/groups | 获取用户群聊列表 |
| POST | /api/groups | 创建群聊 |
| GET | /api/groups/:id/members | 获取群成员列表 |
| POST | /api/groups/:id/members | 添加群成员 |

## 数据库表结构

| 表名 | 描述 |
|------|------|
| users | 用户表 |
| sessions | 好友关系表 |
| messages | 消息表 |
| groups | 群聊表 |
| group_members | 群成员表 |

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.75+
- MySQL 8.0+

### 前端安装与运行

```bash
cd frontend
npm install
npm run dev
```

前端将运行在 http://localhost:3000

### 后端安装与运行

```bash
cd backend
# 配置数据库连接
# 复制 .env 文件并修改配置
cargo run
```

后端将运行在 http://localhost:8080

### 数据库配置

确保 MySQL 服务正在运行，并创建数据库 `html_im`：

```sql
CREATE DATABASE html_im CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

后端启动时会自动执行数据库迁移。

## 开发规范

- 前端组件命名：PascalCase
- 前端文件命名：kebab-case
- 后端命名：snake_case
- 提交规范：`feat:`, `fix:`, `docs:`, `style:`, `refactor:`, `test:`

## 版本历史

### v0.1.0 (当前版本)
- 用户注册和登录功能
- 好友管理（添加、删除、搜索）
- 私聊功能
- 群聊功能（创建群聊、添加群成员）
- WebSocket 实时消息推送
- 在线状态显示

## License

MIT
