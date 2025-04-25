#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if a custom parameter has been set, otherwise use default values
CONTAINER_NAME="postgres"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"
APP_DATABASE__PORT="${POSTGRES_PORT:=5432}"
APP_DATABASE__USERNAME="${APP_DATABASE__USERNAME:=app}"
APP_DATABASE__PASSWORD="${APP_DATABASE__PASSWORD:=secret}"
APP_DATABASE__DATABASE_NAME="${APP_DATABASE__DATABASE_NAME:=empire}"

if ! [ -x "$(command -v diesel)" ]; then
  echo >&2 "Error: diesel is not installed."
  exit 1
fi

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]; then
  # Launch postgres using Docker
  docker run \
  --env POSTGRES_USER="${SUPERUSER}" \
  --env POSTGRES_PASSWORD="${SUPERUSER_PWD}" \
  --env POSTGRESQL_LOG_STATEMENT=all \
  --env POSTGRESQL_LOG_MIN_DURATION_STATEMENT=0 \
  --publish "${APP_DATABASE__PORT}":5432 \
  --detach \
  --restart=always \
  --name "${CONTAINER_NAME}" \
  postgres \
  -c log_statement=all \
  -c log_min_duration_statement=0 \
  -c log_min_messages=info \
  -c log_line_prefix='%t [%p]: [%l-1] player=%u,db=%d ' \
  -N 1000
  # ^ Increased maximum number of connections for testing purposes
  sleep 5

  # Wait for Postgres to be ready to accept connections
  until [ \
    "$(docker inspect -f "{{.State.Status}}" ${CONTAINER_NAME})" == \
    "running" \
  ]; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
  done

  # Create the application player
  CREATE_QUERY="CREATE USER ${APP_DATABASE__USERNAME} WITH PASSWORD '${APP_DATABASE__PASSWORD}';"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

  # Grant create db privileges to the app player
  GRANT_QUERY="ALTER USER ${APP_DATABASE__USERNAME} CREATEDB;"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"
fi
>&2 echo "Postgres is up and running on port ${APP_DATABASE__PORT}!"

export DATABASE_URL=postgres://${APP_DATABASE__USERNAME}:${APP_DATABASE__PASSWORD}@localhost:${APP_DATABASE__PORT}/${APP_DATABASE__DATABASE_NAME}
diesel setup
