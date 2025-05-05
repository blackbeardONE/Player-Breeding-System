#!/bin/sh
set -e

./reset_test_db.sh
./run_migrations.sh
pytest python_microservices/
