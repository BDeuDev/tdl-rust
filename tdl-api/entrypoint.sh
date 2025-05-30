#!/bin/sh
set -e

# Esperar hasta que Postgres esté listo
until PGPASSWORD=password psql -h db -U user -d mydb -c '\q'; do
  >&2 echo "📡 Esperando a que Postgres esté disponible..."
  sleep 1
done

echo "✅ Base de datos lista. Ejecutando migraciones..."
sqlx migrate run

echo "🚀 Iniciando servidor Actix Web..."
exec ./tdl-api
