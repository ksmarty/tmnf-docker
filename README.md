# TMNF-Docker
> Run a TrackMania Nations Forever server easily!

## Setup

1. Create a new user in TMNF. This is your "host user". You can't use your main account because whatever account is specified, won't be able to play.
2. `ADMINS` should be a comma separated list of users. NO SPACES!
2. Fill out `secrets.example`

## Docker Compose

```yaml
version: '3.4'

services:
  tmnfdocker:
    image: ksmarty/tmnf-docker
    container_name: tmnf_server
    volumes:
      - socket:/run/mysqld/
    ports:
      - 2350:2350/tcp # Server
      - 2350:2350/udp # Server
      - 3450:3450/tcp # P2P
      - 3450:3450/udp # P2P
    env_file: ./secrets.example
    restart: unless-stopped
  db:
    image: mariadb
    container_name: tmnf_db
    volumes:
      - socket:/run/mysqld/
    env_file: ./secrets.example
    restart: unless-stopped

networks:
  default:

volumes:
  socket:
```

## Build

```sh
docker build -t ksmarty/tmnf-docker --no-cache --progress plain .
```