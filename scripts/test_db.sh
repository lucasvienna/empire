#!/usr/bin/env bash
set -x
set -eo pipefail

CONTAINER_NAME="${POSTGRES_PORT:=postgres}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"
APP_USER="${APP_USER:=test}"
APP_USER_PWD="${APP_USER_PWD:=test}"
APP_DB_NAME="${APP_DB_NAME:=empire_test}"

if [[ -n "${DO_SETUP}" ]]; then
  # Create the test user
  CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

  # Grant create db privileges to the test user
  GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"
fi

export DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}
diesel database reset
