-- CreateTable
CREATE TABLE "sonarqube_tokens" (
    "id" UUID NOT NULL,
    "name" VARCHAR NOT NULL,
    "token" VARCHAR NOT NULL,
    "project_key" VARCHAR,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expires_at" TIMESTAMPTZ,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    "created_by" VARCHAR,
    "description" TEXT,

    CONSTRAINT "sonarqube_tokens_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "sonarqube_tokens_token_key" ON "sonarqube_tokens"("token");

-- CreateIndex
CREATE INDEX "idx_sonarqube_tokens_name" ON "sonarqube_tokens"("name");

-- CreateIndex
CREATE INDEX "idx_sonarqube_tokens_project_key" ON "sonarqube_tokens"("project_key");

-- CreateIndex
CREATE INDEX "idx_sonarqube_tokens_is_active" ON "sonarqube_tokens"("is_active");

-- CreateIndex
CREATE INDEX "idx_sonarqube_tokens_created_at" ON "sonarqube_tokens"("created_at");

-- CreateIndex
CREATE INDEX "idx_sonarqube_tokens_created_by" ON "sonarqube_tokens"("created_by");
