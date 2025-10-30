# MongoDB 文件資料庫 (MongoDB Document Database)

```sh
$ cargo add mongodb
```

啟用本地開發用資料庫:

```yaml
# compose.yaml
services:
  local-mongo:
    image: mongo:8
    restart: always
    environment:
      MONGO_INITDB_ROOT_USERNAME: root
      MONGO_INITDB_ROOT_PASSWORD: mypassword
      MONGO_INITDB_DATABASE: mydb
    ports:
      - 27017:27017
    volumes:
      - mongo-data:/data

volumes:
  mongo-data:
```

```sh
$ docker compose up local-mongo -d
```

## 分散式運算 (Distributed Computing)

MongoDB Data Federation
