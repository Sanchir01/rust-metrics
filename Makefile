SILENT:
PHONY:
MIGRATION_NAME ?= new_migration
compose:
	docker-compose up -d
dockerfile:
	docker build -t trpc-rust-grpc .

metrics:
	cargo watch -x 'run -p metrics-server'

generator:
	cargo watch -x 'run -p generator'
run-prod:
	cargo run --release -p ingester &  cargo run --release -p generator

workspace:
	cargo watch -x 'run -p generator' & cargo watch -x 'run -p ingester'
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
