DOCKER_COMPOSE_BIN=docker compose
DOCKER_BIN=docker
PROJECT_NAME=sonarcute
IMAGE_VERSION=latest
DOCKER_NETWORK_NAME=sonarcute-network


build-api-image:
	$(DOCKER_BIN) build -t ${PROJECT_NAME}-api:$(IMAGE_VERSION) -f ./deploy/dockerfile/api.Dockerfile ./api

build-web-image:
	$(DOCKER_BIN) build -t ${PROJECT_NAME}-web:$(IMAGE_VERSION) -f ./deploy/dockerfile/web.Dockerfile ./web

build-token-generator-image:
	$(DOCKER_BIN) build -t ${PROJECT_NAME}-token-generator:$(IMAGE_VERSION) -f ./deploy/dockerfile/token.Dockerfile ./deploy

create-network:
	 $(DOCKER_BIN) network inspect $(DOCKER_NETWORK_NAME) >/dev/null 2>&1 || $(DOCKER_BIN) network create $(DOCKER_NETWORK_NAME)

gen-token:
	$(DOCKER_COMPOSE_BIN) -f ./deploy/compose/docker-base-compose.yml -f ./deploy/compose/docker-app-compose.yml -p sonarcute --profile init run --rm tokengenerator

app-base-setup:
	$(DOCKER_COMPOSE_BIN) -f ./deploy/compose/docker-base-compose.yml -f ./deploy/compose/docker-app-compose.yml -p sonarcute up -d

setup: build-web-image build-api-image build-token-generator-image create-network app-base-setup gen-token
