# Redis 鍵值對資料庫 (Redis Key-Value Database)

```sh
$ cargo add redis
```

啟用本地開發用資料庫:

```yaml
# compose.yaml
services:
  local-redis:
    image: redis:8
    restart: always
    command: redis-server --save 60 1 --loglevel warning
    ports:
      - 6379:6379
    volumes:
      - redis-data:/data

volumes:
  redis-data:
```

```sh
$ docker compose up local-redis -d
```

## Cache Deduplication

## Session

## Queues and Workers
