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

echo "Dropping and recreating test database..."
mysql -h db -u user -ppassword -e "DROP DATABASE IF EXISTS playerdb; CREATE DATABASE playerdb;"

echo "Test database reset complete."
