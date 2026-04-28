# HTML-IM 即时通信系统

> 基于 Rust (Actix-web) + Vue 3 + WebSocket 的现代化即时通信应用

![版本](https://img.shields.io/badge/version-0.1.1-blue)
![许可证](https://img.shields.io/badge/license-MIT-green)

## 项目简介

HTML-IM 是一个全栈即时通信系统，支持好友私聊、群聊、消息实时推送等功能。采用现代化的技术栈构建，具有高性能、高可用的特点。

## 功能特性

- **用户认证**：支持注册、登录、Token 认证、验证码验证
- **好友管理**：支持按用户名/邮箱搜索好友、添加好友、删除好友
- **私聊功能**：支持一对一实时消息发送和接收，消息分页加载
- **群聊功能**：支持创建群聊、添加群成员、群消息发送
- **实时通信**：基于 WebSocket 实现消息实时推送
- **在线状态**：实时显示用户在线/离线状态，WebSocket 自动更新
- **消息历史**：支持查看历史消息记录，分页加载防止溢出
- **表情支持**：内置常用表情库
- **管理员面板**：支持查看所有用户、聊天记录管理、一键删除用户/群聊/消息
- **未读消息**：私聊和群聊头像显示未读消息小红点

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
- **验证码**: 自定义验证码生成 (PNG + Base64)

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

### 验证码接口
| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/captcha | 获取验证码图片 |
| POST | /api/captcha/verify | 验证验证码 |

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
| GET | /api/messages | 获取消息列表（支持分页） |
| POST | /api/messages | 发送消息 |
| POST | /api/messages/:id/read | 标记消息为已读 |

### 群聊接口
| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/groups | 获取用户群聊列表 |
| POST | /api/groups | 创建群聊 |
| GET | /api/groups/:id/members | 获取群成员列表 |
| POST | /api/groups/:id/members | 添加群成员 |

### 管理员接口
| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/admin/users | 获取所有用户列表 |
| DELETE | /api/admin/users/:id | 删除指定用户 |
| DELETE | /api/admin/users/delete-all | 一键删除所有非管理员用户 |
| GET | /api/admin/messages | 获取所有消息 |
| GET | /api/admin/conversations | 获取对话列表（按私聊/群聊分类） |
| DELETE | /api/admin/messages/delete-all | 一键删除所有消息 |
| DELETE | /api/admin/messages/delete-selected | 批量删除选中消息 |
| DELETE | /api/admin/groups/delete-all | 一键删除所有群聊 |
| GET | /api/admin/users/:id/messages | 查看指定用户聊天历史 |
| DELETE | /api/admin/users/:id/messages/clear | 清空指定用户聊天记录 |
| GET | /api/admin/groups/:group_id/messages | 查看群聊消息历史 |
| DELETE | /api/admin/groups/:group_id/messages/clear | 清空群聊消息 |

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

### v0.2.0 (当前版本)
- **新增功能**
  - 图片附件上传：支持在聊天中上传图片附件
  - 好友关系验证：发送私聊消息前自动验证双方是否为好友关系
  - 删除好友自动清理：删除好友时自动清除双方聊天记录
  - 实时好友状态同步：删除好友后实时通知对方并自动关闭聊天窗口
  - 添加好友自动跳转：添加好友成功后自动切换到好友列表并选中该好友

- **功能优化**
  - 消息发送权限控制：非好友关系无法发送私聊消息，返回明确错误提示
  - 好友删除数据清理：删除好友时同步删除相关消息记录，保持数据整洁
  - WebSocket 实时通知：好友关系变更时通过 WebSocket 实时推送通知
  - 聊天窗口自动关闭：收到好友被删除通知时自动关闭对应聊天窗口

### v0.1.1
- **新增功能**
  - 验证码功能：登录/注册时支持图形验证码验证
  - 管理员面板：支持查看所有用户、聊天记录管理
  - 一键删除：支持一键删除所有非管理员用户、群聊、消息
  - 聊天记录分类：按私聊和群聊分类显示对话列表
  - 未读消息提示：私聊和群聊头像显示未读消息小红点
  - 消息分页加载：聊天内容支持分页加载，防止内容溢出
  - 动态 WebSocket 地址：根据当前页面地址自动切换 WebSocket 连接地址

- **功能优化**
  - 在线状态实时更新：WebSocket 连接成功后自动更新用户在线状态
  - 添加好友自动刷新：添加好友成功后自动刷新好友列表
  - 群成员离线状态显示：群成员离线状态显示为红色字体
  - 消息去重：修复发送消息时重复显示的问题
  - 管理员面板分页：聊天记录过长时采取分页防止内容溢出

### v0.1.0
- 用户注册和登录功能
- 好友管理（添加、删除、搜索）
- 私聊功能
- 群聊功能（创建群聊、添加群成员）
- WebSocket 实时消息推送
- 在线状态显示

## License

MIT
