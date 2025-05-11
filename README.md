# Verbali authentication

## Running project
This project use [dioxus](https://dioxuslabs.com) framework

1. Create / Copy .env
```bash
cp .env.example .env
```

2. Run by using docker
```bash
docker-compose up -d
```

3. Install node dependencies
```bash
docker compose exec app npm install
```

4. Watch and compile tailwindcss
```bash
docker compose exec app npx tailwindcss -i ./assets/input.css -o ./assets/style.css --watch
```

5. Run DX server
```bash
docker compose exec app dx serve
```

## Database management
We use [Diesel](https://diesel.rs) to interact with postgres database


Create migration
```bash
docker compose exec app diesel migration generate <MIGRATION_FILENAME>
```

Run all migrations needed
```bash
docker compose exec app diesel migration run
```

Redo migration
```bash
docker compose exec app diesel migration redo
```


## Build container
```bash
docker build --network host -t verbali:latest -f dockerfiles/prod.dockerfile .
```
