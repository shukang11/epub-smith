#!/usr/bin/env bash
#
# generate-cover.sh - AI 生图封面生成脚本
#
# 用法：
#   ./scripts/generate-cover.sh --prompt "提示词" --output cover.png
#   ./scripts/generate-cover.sh -p "提示词" -o cover.png
#   echo "提示词" | ./scripts/generate-cover.sh -o cover.png
#
# 环境变量（从 .env 自动加载）：
#   DOGZEE_API_KEY   - API 密钥（必需）
#   DOGZEE_API_URL   - API 端点（可选，默认为 ergouzi）
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# 默认值
DEFAULT_API_URL="https://hk.ergouzi.life/v1/images/generations"
DEFAULT_SIZE="1024x1792"  # 竖版，适合 EPUB 封面

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

usage() {
    cat << EOF
用法: $(basename "$0") [选项]

选项:
  -p, --prompt TEXT    封面提示词（必填，除非从 stdin 读取）
  -o, --output FILE    输出文件路径（默认：cover.png）
  -s, --size SIZE      图片尺寸（默认：1024x1792 竖版）
  -m, --model MODEL    模型名称（默认：gpt-image-2）
  --api-url URL        API 端点（默认：从 .env 读取或使用内置默认值）
  -h, --help           显示此帮助信息

示例:
  # 基本用法
  $(basename "$0") -p "一张科幻风格的封面" -o cover.png

  # 从 stdin 读取提示词
  echo "一张奇幻风格的封面" | $(basename "$0") -o cover.png

  # 指定横版尺寸
  $(basename "$0") -p "提示词" -s 1792x1024 -o cover.png

环境变量:
  DOGZEE_API_KEY   API 密钥（在 .env 文件中配置）
  DOGZEE_API_URL   API 端点地址
EOF
}

log_info() {
    echo -e "${GREEN}✓${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

log_error() {
    echo -e "${RED}✗${NC} $1" >&2
}

# 加载 .env 文件
load_env() {
    local env_file="$PROJECT_DIR/.env"
    if [[ -f "$env_file" ]]; then
        set -a
        source "$env_file"
        set +a
    fi
}

# 解析参数
parse_args() {
    PROMPT=""
    OUTPUT="cover.png"
    SIZE="$DEFAULT_SIZE"
    MODEL="gpt-image-2"
    API_URL=""

    while [[ $# -gt 0 ]]; do
        case $1 in
            -p|--prompt)
                PROMPT="$2"
                shift 2
                ;;
            -o|--output)
                OUTPUT="$2"
                shift 2
                ;;
            -s|--size)
                SIZE="$2"
                shift 2
                ;;
            -m|--model)
                MODEL="$2"
                shift 2
                ;;
            --api-url)
                API_URL="$2"
                shift 2
                ;;
            -h|--help)
                usage
                exit 0
                ;;
            *)
                log_error "未知参数: $1"
                usage
                exit 1
                ;;
        esac
    done

    # 如果没有提供 prompt，尝试从 stdin 读取
    if [[ -z "$PROMPT" ]] && [[ ! -t 0 ]]; then
        PROMPT=$(cat)
    fi

    # 设置 API URL
    if [[ -z "$API_URL" ]]; then
        API_URL="${DOGZEE_API_URL:-$DEFAULT_API_URL}"
    fi
}

# 验证参数
validate() {
    if [[ -z "$PROMPT" ]]; then
        log_error "缺少提示词。使用 -p 指定或通过 stdin 传入。"
        exit 1
    fi

    if [[ -z "${DOGZEE_API_KEY:-}" ]] || [[ "$DOGZEE_API_KEY" == "your-key-here" ]]; then
        log_error "缺少 API 密钥。请在 .env 文件中设置 DOGZEE_API_KEY。"
        exit 1
    fi
}

# 生成封面
generate_cover() {
    local prompt="$1"
    local output="$2"
    local size="$3"
    local model="$4"
    local api_url="$5"

    log_info "正在生成封面..."
    log_info "模型: $model | 尺寸: $size"
    log_info "提示词: ${prompt:0:80}..."

    # 调用 API
    local response
    response=$(curl -s --request POST \
        --url "$api_url" \
        --header "Content-Type: application/json" \
        --header "Authorization: Bearer $DOGZEE_API_KEY" \
        --data "$(jq -n \
            --arg model "$model" \
            --arg prompt "$prompt" \
            --arg size "$size" \
            '{
                model: $model,
                prompt: $prompt,
                size: $size,
                n: 1
            }')")

    # 检查错误
    if echo "$response" | jq -e '.error' >/dev/null 2>&1; then
        local error_msg
        error_msg=$(echo "$response" | jq -r '.error.message // .error')
        log_error "API 错误: $error_msg"
        return 1
    fi

    # 提取并保存图片
    local b64_data
    b64_data=$(echo "$response" | jq -r '.data[0].b64_json')

    if [[ -z "$b64_data" ]] || [[ "$b64_data" == "null" ]]; then
        log_error "未能获取图片数据"
        return 1
    fi

    echo "$b64_data" | base64 -d > "$output"

    # 验证输出文件
    if [[ ! -f "$output" ]]; then
        log_error "文件保存失败: $output"
        return 1
    fi

    local file_size
    file_size=$(ls -lh "$output" | awk '{print $5}')
    log_info "封面已保存: $output ($file_size)"

    return 0
}

# 主函数
main() {
    load_env
    parse_args "$@"
    validate

    if generate_cover "$PROMPT" "$OUTPUT" "$SIZE" "$MODEL" "$API_URL"; then
        log_info "完成！"
        exit 0
    else
        log_error "生成失败"
        exit 1
    fi
}

main "$@"
