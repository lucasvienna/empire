#!/usr/bin/bash
set -x
set -eo pipefail

PATTERN="^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
CONTAINER_NAME="${POSTGRES_PORT:=postgres}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"

QUERY="SELECT 'DROP DATABASE ' || quote_ident(datname) || ';' FROM pg_database WHERE datname ~ '${PATTERN}';"
CMD="psql -U ${SUPERUSER} -Atqc \"${QUERY}\" | psql -U ${SUPERUSER}"

docker exec -it "${CONTAINER_NAME}" bash -c "${CMD}"
