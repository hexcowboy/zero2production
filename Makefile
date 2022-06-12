# Load environment variables from .env
ifneq (,$(wildcard ./.env))
	include .env
	export
endif

# Export default environment variables
DB_USER ?= postgres
DB_PASS ?= password
DB_NAME ?= zeroprod
DB_PORT ?= 5432

# Hook to chck if command exists
cmd-exists-%:
	@hash $(*) > /dev/null 2>&1 || \
		(echo "ERROR: '$(*)' must be installed and available on your PATH."; exit 1)

# Hook to wait for Postgres to be available
wait-for-postgres: cmd-exists-psql
	@until $$(PGPASSWORD=${DB_PASS} psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'); do \
		@echo "Postgres is still unavailable - sleeping"; \
		sleep 1; \
	done
	@echo "Postgres is up and running on port ${DB_PORT}!"

.PHONY: db-create
db-create:
	docker run \
		-e POSTGRES_USER="${DB_USER}" \
		-e POSTGRES_PASSWORD="${DB_PASS}" \
		-e POSTGRES_DB="${DB_NAME}" \
		-p "${DB_PORT}":5432 \
		-d postgres:14-alpine \
		postgres -N 1000

.PHONY: db-init
db-init: wait-for-postgres cmd-exists-sqlx
	sqlx database create -D "postgres://${DB_USER}:${DB_PASS}@localhost:${DB_PORT}/${DB_NAME}"

.PHONY: db-drop
db-drop: cmd-exists-docker
	docker exec -it postgres dropdb ${DB_NAME}

.PHONY: migrate
migrate: wait-for-postgres cmd-exists-sqlx
	sqlx migrate run
