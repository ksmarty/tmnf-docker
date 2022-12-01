# TMNF-Docker

> Run a **TrackMania Nations Forever** server easily!

## Setup

Create a new user in TMNF. This is your "host user". You can't use your main account because whatever account is specified, won't be able to play.

Edit `secrets.example` based on variables below

## Environment Variables

❗ = Required

### General

| Variable              | Default            | Allowed Values | Notes                                                    |
| --------------------- | ------------------ | -------------- | -------------------------------------------------------- |
| `SERVER_NAME`         | TMNF Docker Server | String         |                                                          |
| `NATION`              | ❗                 | String{3}      | 3-letter country abbreviation                            |
| `SERVER_PASS`         | P@ssw0rd123        | String         | Set as empty string for no password                      |
| `HOST_USER`           | ❗                 | String         | Username of host TMNF account                            |
| `HOST_PASS`           | ❗                 | String         | Password of host TMNF account                            |
| `ADMINS`              |                    | String         | Comma-separated list of admin usernames (**No spaces!**) |
| `SERVER_PORT`         | 2350               | Number         |                                                          |
| `P2P_PORT`            | 3450               | Number         |                                                          |
| `RPC_PORT`            | 5000               | Number         |                                                          |
| `MYSQL_DATABASE`      | aseco              | String         |                                                          |
| `MYSQL_USER`          | tmf                | String         |                                                          |
| `MYSQL_PASSWORD`      | MYSQL_P4SS         | String         |                                                          |
| `MYSQL_ROOT_PASSWORD` | MYSQL_R00T_P4SS    | String         |                                                          |

### Custom Game Mode

| Variable           | Default | Allowed Values | Notes                                                                                              |
| ------------------ | ------- | -------------- | -------------------------------------------------------------------------------------------------- |
| `AUTOSAVE`         | `OFF`   | `ON` / `OFF`   | Enable / Disable autosaves                                                                         |
| `GAME_CONFIG`      |         | String         | File in `GameData/Tracks/MatchSettings/Nations`. `Custom.xml` is pre-installed, or create your own |
| `GAME_MODE`        |         | String         | One of: `Rounds`, `TimeAttack`, `Team`, `Laps`, `Stunts`                                           |
| `RANDOM_MAP_ORDER` | `OFF`   | `ON` / `OFF`   | Shuffle map order on startup                                                                       |

#### Mode-Specific Settings

| Variable             | Mode          | Allowed Values           | Notes                                       |
| -------------------- | ------------- | ------------------------ | ------------------------------------------- |
| `ROUNDS_POINTSLIMIT` | 🏎️ Rounds     | Number                   | Points limit                                |
| `ROUNDS_USENEWRULES` | 🏎️ Rounds     | `1` (True) / `0` (False) | New rules are used for rounds mode          |
| `ROUNDS_FORCEDLAPS`  | 🏎️ Rounds     | Number                   | Number of laps (Default: 0 or set by track) |
| `TIMEATTACK_LIMIT`   | ⏰ TimeAttack | Number                   | Time limit in milliseconds                  |
| `TEAM_POINTSLIMIT`   | 👨‍👩‍👧‍👦 Team       | Number                   | Points limit                                |
| `TEAM_MAXPOINTS`     | 👨‍👩‍👧‍👦 Team       | Number                   | Number of maximum points per round          |
| `TEAM_USENEWRULES`   | 👨‍👩‍👧‍👦 Team       | `1` (True) / `0` (False) | New rules are used for rounds mode          |
| `LAPS_NBLAPS`        | 🏁 Laps       | Number                   | Number of laps                              |
| `LAPS_TIMELIMIT`     | 🏁 Laps       | Number                   | Time limit in milliseconds                  |

## Docker Compose

Copy config from [docker-compose.yml](./docker-compose.yml). Optionally, uncomment lines `9`, `31-36` and edit line `35` to map to server files to your host fs.

## Build

```sh
docker build -t ksmarty/tmnf-docker --no-cache --progress plain .
```
