if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
fi

if [ -z "$DATABASE_URL" ]; then
  exit 1
fi

sea-orm-cli migrate up -u "$DATABASE_URL"

sea-orm-cli generate entity -u "$DATABASE_URL" -o src/entities


echo "Done!"