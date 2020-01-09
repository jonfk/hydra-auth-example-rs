#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER hydra WITH PASSWORD 'hello';
    CREATE USER auth WITH PASSWORD 'authpassword';
    CREATE DATABASE hydra;
    CREATE DATABASE auth;
    GRANT ALL PRIVILEGES ON DATABASE hydra TO hydra;
    GRANT ALL PRIVILEGES ON DATABASE auth TO auth;
EOSQL
