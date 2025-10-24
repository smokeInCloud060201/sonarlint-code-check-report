DOCKER_COMPOSE_BIN=docker compose

setup:
	$(DOCKER_COMPOSE_BIN) up -d

migrate:
	cd api && migrate.bat up

migrate-down:
	cd api && migrate.bat down

migrate-status:
	cd api && migrate.bat status
	