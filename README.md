# home_budget

Containerized read-in and storage of budget data

## Purpose

Pretty simple - I've been keeping my home budget in an Excel spreadsheet for years, it's about time it was stored in a db.  Plus I wanted to do more Rust stuff.

## Pre-requisites

- [Docker installation](https://docs.docker.com/engine/install/)
- [Diesel ORM](https://diesel.rs/guides/getting-started.html)

...I think that's it...if I have to tell you to install Rust then :shrug:

## Usage

1. Create `.env` file:

    ```bash
    BUDGET_FILEPATH={PATH TO OLD BUDGET WORKBOOK}

    DATABASE_URL=postgres://{USERNAME}:{PASSWORD}@localhost/{DB NAME}

    POSTGRES_USER={USERNAME}
    POSTGRES_PASSWORD={PASSWORD}
    POSTGRES_DB={DB NAME}
    ```

1. Spin up your postgres container
  
    - `docker-compose up -d`

1. Run your migrations

    - `diesel migration run`

1. Run the `upload_all` to load in all worksheets found at `BUDGET_FILEPATH`

    - `cargo run --bin upload_all`

    - NOTE: Worksheet names are expected to have the format `%B %Y` e.g. `November 2022` [Rust Date Formats](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
