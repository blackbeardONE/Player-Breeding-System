#!/bin/sh
set -e

echo "Waiting for database to be ready..."

MAX_RETRIES=60
RETRY_COUNT=0

until mysql -h db -u user -ppassword -e "SELECT 1" playerdb > /dev/null 2>&1; do
  RETRY_COUNT=$((RETRY_COUNT + 1))
  echo "Database not ready yet. Retry $RETRY_COUNT/$MAX_RETRIES..."
  if [ $RETRY_COUNT -ge $MAX_RETRIES ]; then
    echo "Error: Database not ready after $MAX_RETRIES attempts. Exiting."
    exit 1
  fi
  sleep 1
done

echo "Database is ready. Applying migrations..."
for file in /app/migrations/*.sql; do
  echo "Applying $file"
  mysql -h db -u user -ppassword playerdb < "$file"
done

echo "Migrations applied successfully."
