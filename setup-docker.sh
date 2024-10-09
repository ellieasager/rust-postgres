#!/bin/bash


docker volume create db-data

# create the network
docker network create postgresnet

docker run --rm -d --mount \
  "type=volume,src=db-data,target=/var/lib/postgresql/data" \
  -p 5432:5432 \
  --network postgresnet \
  --name db \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=messages \
  postgres

# # pull postgres image
# docker pull postgres:17.0

# build the main container
docker build -t elliecat/rust-postgres .

docker run \
  --rm -d \
  --network postgresnet \
  --name docker-develop-rust-container \
  -p 3001:8080 \
  -e PG_DBNAME=messages \
  -e PG_HOST=db \
  -e PG_USER=postgres \
  -e PG_PASSWORD=postgres \
  -e ADDRESS=0.0.0.0:8080 \
  -e RUST_LOG=debug \
  rust-backend-image


# start the postgres container
docker run --rm -d --mount \
  "type=volume,src=db-data,target=/var/lib/postgresql/data" \
  -p 5432:5432 \
  --network rust-postgres \
  --name postgresdb \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=messages \
  postgres

# start the go app container





# docker run -d --net rust-postgres --name rust-postgres -p 5432:5432 elliecat/rust-postgres