source .env
diesel database reset --database-url $DEV_DATABASE_URL
cargo test
