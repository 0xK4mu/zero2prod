#! /usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
	echo >&2 "Error: psql is not installed."
	exit 1
fi
if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "Error: sqlx is not installed."
	echo >&2 "Use:"
	echo >&2 " cargo install --version='~0.7' sqlx-cli \
--no-default-features --features rustls,postgres"
	echo >&2 "to install it."
	exit 1
fi

# Vérifier si un user custom a été set, sinon utiliser 'postgres' pâr défaut
DB_USER="${POSTGRES_USER:=postgres}"

# Vérifier si un password a été set, sinon utiliser 'password' par défaut
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

# Vérifier si un nom de DB custom a été set, sinon utiliser 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"

# Vérifier si un port custom a été set, sino utiliser '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Vérifier si un host custom a été set, sinon utiliser 'localhost' par défaut
DB_HOST="${POSTGRES_HOST:=localhost}"

# Allow to skip Docker if a dockjerized Postgres database is already running
# Lancer postgre avec Docker
if [[ -z "${SKIP_DOCKER}" ]]
then
	docker run \
		-e POSTGRES_USER=${DB_USER} \
		-e POSTGRES_PASSWORD=${DB_PASSWORD} \
		-e POSTGRES_DB=${DB_NAME} \
		-p "${DB_PORT}":5432 \
		-d postgres \
		postgres -N 1000
	# ^ augmente le nombre maximum de connexion dans le cadre des tests
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWOWD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
	>&2 echo "Postgres is still unavailable - sleeping"
	sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"


