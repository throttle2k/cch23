#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql command is not installed"
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx command is not installed"
  exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=postgres}"
DB_PORT="${POSTGRES_PORT:=16695}"

if [[ -z "${SKIP_DOCKER}" ]]
then
  docker pod create --name postgres -p 9876:80 -p 16695:5432
  docker run --pod="postgres" \
    -e PGADMIN_DEFAULT_EMAIL="admin@email.com" \
    -e PGADMIN_DEFAULT_PASSWORD="password" \
    --name pgadmin \
    -d dpage/pgadmin4
  docker run --pod="postgres" \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    --name postgres \
    -d postgres
      #    -p "${DB_PORT}":5432 \
fi

# Keeps pinging postgres until ready
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres still unavailable - back to sleep"
  sleep 1 
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run 

>&2 echo "Postgres database has been migrated, ready to go!"
