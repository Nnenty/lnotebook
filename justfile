prepare:
    cargo sqlx prepare -- --all-targets

# Use this after you have specified the `DATABASE_URL`
migrate:
    cd ../notebook_api && sqlx migrate run