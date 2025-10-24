# Database Migration Setup Guide

## Environment Variables

The migration system requires the `DATABASE_URL` environment variable to be set. Here are the different ways to set it up:

### Option 1: Using Makefile (Recommended)
The Makefile now uses cross-platform migration scripts that automatically set the DATABASE_URL:

```bash
make migrate-status    # Check migration status
make migrate          # Run migrations
make migrate-down     # Rollback migrations
```

**How it works:**
- Windows: Uses `migrate.bat` (batch file)
- Unix/Linux/Git Bash: Uses `migrate.sh` (shell script)
- Both scripts automatically set the DATABASE_URL environment variable

### Option 2: Manual Environment Variable
Set the environment variable manually before running migrations:

**Windows PowerShell:**
```powershell
$env:DATABASE_URL="postgresql://sonar:sonar@localhost:5432/sonarqube"
cd api
sea-orm-cli migrate status
```

**Windows Command Prompt:**
```cmd
set DATABASE_URL=postgresql://sonar:sonar@localhost:5432/sonarqube
cd api
sea-orm-cli migrate status
```

**Linux/Mac:**
```bash
export DATABASE_URL="postgresql://sonar:sonar@localhost:5432/sonarqube"
cd api
sea-orm-cli migrate status
```

### Option 3: Direct Script Usage
You can also run the migration scripts directly:

**Windows (PowerShell/CMD):**
```bash
cd api
./migrate.bat status
./migrate.bat up
./migrate.bat down
```

**Unix/Linux/Git Bash:**
```bash
cd api
./migrate.sh status
./migrate.sh up
./migrate.sh down
```

### Option 4: Create .env file
Create a `.env` file in the `api` directory:

```bash
# In api/.env
DATABASE_URL=postgresql://sonar:sonar@localhost:5432/sonarqube
```

## Database Connection Details

- **Host**: localhost
- **Port**: 5432
- **Database**: sonarqube
- **Username**: sonar
- **Password**: sonar

## Migration Commands

| Command | Description |
|---------|-------------|
| `make migrate-status` | Check current migration status |
| `make migrate` | Apply all pending migrations |
| `make migrate-down` | Rollback the last migration |

## Troubleshooting

1. **"Environment variable 'DATABASE_URL' not set"**
   - Use one of the options above to set the DATABASE_URL
   - Make sure the database is running (`docker ps`)

2. **"Connection refused"**
   - Ensure PostgreSQL is running: `docker ps`
   - Start the database: `make setup`

3. **"Database does not exist"**
   - The database should be created automatically by SonarQube
   - Check if SonarQube container is running
