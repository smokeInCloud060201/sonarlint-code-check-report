-- CreateTable
CREATE TABLE "projects" (
    "id" UUID NOT NULL,
    "sonarqube_key" VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    "visibility" VARCHAR NOT NULL,
    "qualifier" VARCHAR NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "sonarqube_created_at" TIMESTAMPTZ,
    "description" TEXT,
    "language" VARCHAR,
    "tags" TEXT,
    "is_active" BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT "projects_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "projects_sonarqube_key_key" ON "projects"("sonarqube_key");

-- CreateIndex
CREATE INDEX "idx_projects_sonarqube_key" ON "projects"("sonarqube_key");

-- CreateIndex
CREATE INDEX "idx_projects_name" ON "projects"("name");

-- CreateIndex
CREATE INDEX "idx_projects_is_active" ON "projects"("is_active");

-- CreateIndex
CREATE INDEX "idx_projects_created_at" ON "projects"("created_at");
