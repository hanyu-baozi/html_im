#!/bin/bash

# ============================================================================
# HTML-IM 项目自动更新脚本
# 用途：一键更新前后端代码、编译、构建、迁移数据库、重启服务
# 使用：chmod +x update.sh && ./update.sh
# ============================================================================

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置变量
PROJECT_DIR="/root/projects/html-im"
BACKEND_DIR="$PROJECT_DIR/backend"
FRONTEND_DIR="$PROJECT_DIR/frontend"
MIGRATIONS_DIR="$BACKEND_DIR/migrations"
BACKUP_DIR="/root/backups"
DATE=$(date +%Y%m%d_%H%M%S)
LOG_FILE="/tmp/html-im-update-$DATE.log"

# MySQL 配置
MYSQL_USER="root"
MYSQL_PASSWORD="123456"
MYSQL_DATABASE="html_im"

# Git 配置
GIT_BRANCH="main"

# ============================================================================
# 函数定义
# ============================================================================

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$LOG_FILE"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_FILE"
}

print_separator() {
    echo -e "${BLUE}========================================${NC}" | tee -a "$LOG_FILE"
}

# 检查命令是否存在
check_command() {
    if ! command -v $1 &> /dev/null; then
        print_error "$1 未安装，请先安装"
        exit 1
    fi
}

# 备份数据库
backup_database() {
    print_separator
    print_info "开始备份数据库..."
    
    mkdir -p "$BACKUP_DIR"
    
    local backup_file="$BACKUP_DIR/html_im_$DATE.sql"
    
    if mysqldump -u "$MYSQL_USER" -p"$MYSQL_PASSWORD" "$MYSQL_DATABASE" > "$backup_file" 2>>"$LOG_FILE"; then
        print_success "数据库备份成功: $backup_file"
        
        # 压缩备份文件
        gzip "$backup_file"
        print_success "备份文件已压缩: ${backup_file}.gz"
    else
        print_error "数据库备份失败！"
        exit 1
    fi
    
    # 清理 7 天前的备份
    print_info "清理 7 天前的备份文件..."
    find "$BACKUP_DIR" -name "html_im_*.sql.gz" -mtime +7 -delete 2>>"$LOG_FILE"
    print_success "旧备份清理完成"
}

# 备份前端文件
backup_frontend() {
    print_separator
    print_info "开始备份前端文件..."
    
    if [ -d "$FRONTEND_DIR/dist" ]; then
        local backup_file="$BACKUP_DIR/frontend_$DATE.tar.gz"
        tar -czf "$backup_file" -C "$FRONTEND_DIR" dist 2>>"$LOG_FILE"
        print_success "前端备份成功: $backup_file"
        
        # 清理旧备份
        find "$BACKUP_DIR" -name "frontend_*.tar.gz" -mtime +7 -delete 2>>"$LOG_FILE"
    else
        print_warning "前端 dist 目录不存在，跳过备份"
    fi
}

# 拉取最新代码
pull_code() {
    print_separator
    print_info "拉取最新代码 (分支: $GIT_BRANCH)..."
    
    cd "$PROJECT_DIR"
    
    # 检查是否有未提交的更改
    if [ -n "$(git status --porcelain)" ]; then
        print_warning "检测到未提交的更改，正在暂存..."
        git stash 2>>"$LOG_FILE"
    fi
    
    # 拉取代码
    if git pull origin "$GIT_BRANCH" 2>>"$LOG_FILE"; then
        print_success "代码拉取成功"
    else
        print_error "代码拉取失败！"
        exit 1
    fi
}

# 更新后端
update_backend() {
    print_separator
    print_info "开始更新后端..."
    
    cd "$BACKEND_DIR"
    
    # 检查 Cargo.toml 是否有变化
    if git diff HEAD~1 Cargo.toml 2>/dev/null | grep -q "dependencies"; then
        print_info "检测到依赖变化，更新 Cargo.lock..."
        cargo update 2>>"$LOG_FILE"
    fi
    
    # 编译后端
    print_info "编译后端 (release 模式)..."
    if cargo build --release 2>>"$LOG_FILE"; then
        print_success "后端编译成功"
    else
        print_error "后端编译失败！查看日志: $LOG_FILE"
        exit 1
    fi
    
    # 检查可执行文件
    if [ -f "$BACKEND_DIR/target/release/html-im-backend" ]; then
        print_success "后端可执行文件已生成"
    else
        print_error "后端可执行文件未找到！"
        exit 1
    fi
}

# 更新前端
update_frontend() {
    print_separator
    print_info "开始更新前端..."
    
    cd "$FRONTEND_DIR"
    
    # 检查 package.json 是否有变化
    if git diff HEAD~1 package.json 2>/dev/null | grep -q "dependencies"; then
        print_info "检测到依赖变化，安装新依赖..."
        npm install --legacy-peer-deps 2>>"$LOG_FILE"
        print_success "前端依赖安装完成"
    else
        print_info "依赖无变化，跳过安装"
    fi
    
    # 构建前端
    print_info "构建前端..."
    if npm run build 2>>"$LOG_FILE"; then
        print_success "前端构建成功"
    else
        print_error "前端构建失败！查看日志: $LOG_FILE"
        exit 1
    fi
    
    # 检查构建产物
    if [ -d "$FRONTEND_DIR/dist" ]; then
        print_success "前端构建产物已生成"
    else
        print_error "前端构建产物未找到！"
        exit 1
    fi
}

# 执行数据库迁移
run_migrations() {
    print_separator
    print_info "检查数据库迁移..."
    
    cd "$MIGRATIONS_DIR"
    
    # 获取已执行的迁移文件列表（从数据库）
    local executed_migrations=$(mysql -u "$MYSQL_USER" -p"$MYSQL_PASSWORD" -N -B -e \
        "SELECT migration_name FROM schema_migrations ORDER BY applied_at;" \
        "$MYSQL_DATABASE" 2>/dev/null || echo "")
    
    local migration_count=0
    
    # 检查每个 SQL 文件
    for sql_file in *.sql; do
        if [ "$sql_file" = "*.sql" ]; then
            print_warning "未找到迁移文件"
            break
        fi
        
        # 跳过 setup_database.sql（只创建数据库，不创建表）
        if [ "$sql_file" = "setup_database.sql" ]; then
            continue
        fi
        
        # 检查是否已执行
        if echo "$executed_migrations" | grep -q "$sql_file"; then
            print_info "跳过已执行的迁移: $sql_file"
            continue
        fi
        
        # 执行迁移
        print_info "执行迁移: $sql_file"
        if mysql -u "$MYSQL_USER" -p"$MYSQL_PASSWORD" "$MYSQL_DATABASE" < "$sql_file" 2>>"$LOG_FILE"; then
            # 记录迁移
            mysql -u "$MYSQL_USER" -p"$MYSQL_PASSWORD" -e \
                "INSERT INTO schema_migrations (migration_name, applied_at) VALUES ('$sql_file', NOW());" \
                "$MYSQL_DATABASE" 2>>"$LOG_FILE" || true
            
            migration_count=$((migration_count + 1))
            print_success "迁移成功: $sql_file"
        else
            print_error "迁移失败: $sql_file"
            exit 1
        fi
    done
    
    if [ $migration_count -eq 0 ]; then
        print_success "所有迁移已执行，无需更新"
    else
        print_success "成功执行 $migration_count 个迁移"
    fi
}

# 创建迁移记录表（如果不存在）
create_migration_table() {
    mysql -u "$MYSQL_USER" -p"$MYSQL_PASSWORD" -e "
    CREATE TABLE IF NOT EXISTS schema_migrations (
        id INT AUTO_INCREMENT PRIMARY KEY,
        migration_name VARCHAR(255) NOT NULL UNIQUE,
        applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );" "$MYSQL_DATABASE" 2>>"$LOG_FILE" || true
}

# 重启服务
restart_services() {
    print_separator
    print_info "重启服务..."
    
    # 停止后端服务
    print_info "停止后端服务..."
    systemctl stop html-im-backend 2>>"$LOG_FILE" || true
    
    # 杀掉可能占用端口的进程
    print_info "清理端口占用..."
    fuser -k 8080/tcp 2>>"$LOG_FILE" || true
    sleep 2
    
    # 启动后端服务
    print_info "启动后端服务..."
    if systemctl start html-im-backend 2>>"$LOG_FILE"; then
        print_success "后端服务启动成功"
    else
        print_error "后端服务启动失败！"
        systemctl status html-im-backend 2>>"$LOG_FILE"
        exit 1
    fi
    
    # 等待后端启动
    print_info "等待后端启动 (5 秒)..."
    sleep 5
    
    # 检查后端是否正常运行
    if curl -s http://localhost:8080/api/captcha > /dev/null 2>&1; then
        print_success "后端 API 响应正常"
    else
        print_warning "后端 API 未响应，请检查日志: journalctl -u html-im-backend -f"
    fi
    
    # 重新加载 Nginx
    print_info "重新加载 Nginx..."
    if nginx -t 2>>"$LOG_FILE" && systemctl reload nginx 2>>"$LOG_FILE"; then
        print_success "Nginx 重新加载成功"
    else
        print_error "Nginx 重新加载失败！"
        exit 1
    fi
}

# 验证更新
verify_update() {
    print_separator
    print_info "验证更新结果..."
    
    local errors=0
    
    # 检查后端服务状态
    if systemctl is-active --quiet html-im-backend; then
        print_success "后端服务运行正常"
    else
        print_error "后端服务未运行！"
        errors=$((errors + 1))
    fi
    
    # 检查 Nginx 状态
    if systemctl is-active --quiet nginx; then
        print_success "Nginx 运行正常"
    else
        print_error "Nginx 未运行！"
        errors=$((errors + 1))
    fi
    
    # 测试后端 API
    if curl -sf http://localhost:8080/api/captcha > /dev/null 2>&1; then
        print_success "后端 API 测试通过"
    else
        print_error "后端 API 测试失败！"
        errors=$((errors + 1))
    fi
    
    # 测试 Nginx 代理
    if curl -sf http://localhost/api/captcha > /dev/null 2>&1; then
        print_success "Nginx 代理测试通过"
    else
        print_error "Nginx 代理测试失败！"
        errors=$((errors + 1))
    fi
    
    # 检查前端文件
    if [ -f "$FRONTEND_DIR/dist/index.html" ]; then
        print_success "前端文件存在"
    else
        print_error "前端文件缺失！"
        errors=$((errors + 1))
    fi
    
    return $errors
}

# 清理临时文件
cleanup() {
    print_separator
    print_info "清理临时文件..."
    
    # 清理 npm 缓存
    cd "$FRONTEND_DIR"
    npm cache clean --force 2>>"$LOG_FILE" || true
    
    # 清理 cargo 缓存（可选，注释掉以保留缓存加速下次编译）
    # cargo cache clean 2>>"$LOG_FILE" || true
    
    print_success "清理完成"
}

# 显示更新摘要
show_summary() {
    print_separator
    print_success "更新完成！"
    print_separator
    
    echo -e "${GREEN}更新摘要:${NC}" | tee -a "$LOG_FILE"
    echo "  更新时间: $(date '+%Y-%m-%d %H:%M:%S')" | tee -a "$LOG_FILE"
    echo "  Git 分支: $GIT_BRANCH" | tee -a "$LOG_FILE"
    echo "  最后提交: $(cd $PROJECT_DIR && git log -1 --oneline)" | tee -a "$LOG_FILE"
    echo "  后端版本: $(cd $PROJECT_DIR && git log -1 --format='%h %s')" | tee -a "$LOG_FILE"
    echo "  备份目录: $BACKUP_DIR" | tee -a "$LOG_FILE"
    echo "  日志文件: $LOG_FILE" | tee -a "$LOG_FILE"
    
    echo ""
    echo -e "${BLUE}访问地址:${NC}" | tee -a "$LOG_FILE"
    echo "  前端: http://120.79.164.201" | tee -a "$LOG_FILE"
    echo "  后端: http://120.79.164.201:8080" | tee -a "$LOG_FILE"
    
    echo ""
    echo -e "${BLUE}常用命令:${NC}" | tee -a "$LOG_FILE"
    echo "  查看后端日志: journalctl -u html-im-backend -f" | tee -a "$LOG_FILE"
    echo "  查看 Nginx 日志: tail -f /var/log/nginx/error.log" | tee -a "$LOG_FILE"
    echo "  重启服务: systemctl restart html-im-backend && systemctl reload nginx" | tee -a "$LOG_FILE"
    
    print_separator
}

# ============================================================================
# 主流程
# ============================================================================

main() {
    print_separator
    print_info "HTML-IM 项目自动更新脚本"
    print_info "开始时间: $(date '+%Y-%m-%d %H:%M:%S')"
    print_separator
    
    # 检查必要命令
    print_info "检查必要命令..."
    check_command git
    check_command cargo
    check_command npm
    check_command mysql
    check_command systemctl
    check_command nginx
    print_success "所有必要命令已安装"
    
    # 检查是否在正确的目录
    if [ ! -d "$PROJECT_DIR" ]; then
        print_error "项目目录不存在: $PROJECT_DIR"
        exit 1
    fi
    
    # 1. 备份
    backup_database
    backup_frontend
    
    # 2. 拉取代码
    pull_code
    
    # 3. 更新后端
    update_backend
    
    # 4. 更新前端
    update_frontend
    
    # 5. 数据库迁移
    create_migration_table
    run_migrations
    
    # 6. 重启服务
    restart_services
    
    # 7. 验证
    if verify_update; then
        print_success "所有验证通过！"
    else
        print_warning "部分验证失败，请检查日志"
    fi
    
    # 8. 清理
    cleanup
    
    # 9. 显示摘要
    show_summary
    
    print_success "更新脚本执行完成！"
    print_info "日志文件: $LOG_FILE"
}

# 错误处理
trap 'print_error "脚本执行失败！查看日志: $LOG_FILE"; exit 1' ERR

# 执行主流程
main "$@"
