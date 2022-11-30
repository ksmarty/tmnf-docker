# TMNF-Docker

> Run a **TrackMania Nations Forever** server easily!

## Setup

1. Create a new user in TMNF. This is your "host user". You can't use your main account because whatever account is specified, won't be able to play.
2. `ADMINS` should be a comma separated list of users. NO SPACES!
3. Fill out `secrets.example`

## Docker Compose

Copy config from [docker-compose.yml](./docker-compose.yml). Optionally, uncomment lines `9`, `31-36` and edit line `35` to map to server files to your host fs.

## Build

```sh
docker build -t ksmarty/tmnf-docker --no-cache --progress plain .
```
