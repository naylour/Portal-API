#!/usr/bin/env bash
# scripts.sh
# Универсальный скрипт для работы с docker-compose и env

# ---- Конфигурация проекта ----
ENV_FILE=".env"
COMPOSE_FILES="-f infra/docker-compose.yaml -f infra/docker-compose.dev.yaml"

# ---- Проверка аргументов ----
if [ $# -lt 1 ]; then
    echo "Usage: $0 <command> [args...]"
    echo "Commands:"
    echo "  ollama <ollama-args>       Run Ollama commands in agent container"
    echo "  exec <service> <cmd...>    Run any command in a service"
    echo "  up                         docker compose up -d"
    echo "  down                       docker compose down"
    exit 1
fi

COMMAND=$1
shift

# ---- Функция для docker compose с env и файлами ----
dc() {
    docker compose --env-file "$ENV_FILE" $COMPOSE_FILES "$@"
}

# ---- Команды ----
case "$COMMAND" in
    agent)
        # Все что после ollama передаем в контейнер agent
        dc exec -it agent ollama "$@"
        ;;
    exec)
        # exec <service> <cmd...>
        SERVICE=$1
        shift
        dc exec -it "$SERVICE" "$@"
        ;;
    up)
        dc up -d
        ;;
    down)
        dc down
        ;;
    *)
        echo "Unknown command: $COMMAND"
        exit 1
        ;;
esac
