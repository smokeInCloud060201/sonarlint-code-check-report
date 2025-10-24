@echo off
set DATABASE_URL=postgresql://sonar:sonar@localhost:5432/sonarqube
sea-orm-cli migrate %*
