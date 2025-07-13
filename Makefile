SILENT:
PHONY:
MIGRATION_NAME ?= new_migration
compose:
	docker-compose up -d

metrics:
	cargo watch -x 'run -p metrics-server'

url	:
	cargo watch -x 'run -p url-shortener'

run-prod:
	cargo run --release -p metrics

workspace:
	cargo watch -x 'run -p metrics-server' && cargo watch -x 'run -p url-shortener'

migrations-up:
	goose -dir migrations clickhouse "tcp://localhost:9000?username=default&password=clickhouse" up

migrations-down:
	goose -dir migrations postgres  "host=localhost user=postgres password=postgres port=5435 dbname=test sslmode=disable"  down


migrations-status:
	goose -dir migrations postgres  "host=localhost user=postgres password=postgres port=5435 dbname=test sslmode=disable" status
migrations-new:
	goose -dir migrations create $(MIGRATION_NAME) sql

docker:
	docker compose up -d

compose-prod:
	docker compose -f docker-compose.prod.yaml up --build -d
