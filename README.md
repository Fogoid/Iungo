# Iungo

Project of a social media app which will consist of the following components:
- Backend
- Consumer
- Frontend (In roadmap)

The focus of this project is to learn different technologies, such as:
- Rust
- MySql
- Docker
- React

## Deployability

### Dockerfiles and Docker Compose
You can find in the root of the project a compose.yml file. This file is a Docker Compose file which allows a quick setup of the required services to run all the project.
So far, the Docker Compose file is able to start the following services:
- MySql database (db)
- Adminer (db manager, used for debugging purposes)
- Backend (built using the Dockerfile which can be found in backend/.dockerfile)

You can run the project by executing on the command line `docker compose up`

### Migrations
You can also find in the root of this project a migrations folder. 
In this folder, you can find the list of sql scripts containing all the changes over time to the database structure.

To run the migrations against the database, you need to have installed [sqlx-cli](https://lib.rs/crates/sqlx-cli) and execute the following instructions:
1. `sqlx database create`
2. `sqlx migrate run`
