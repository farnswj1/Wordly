# Wordly
This is a recreation of Wordle, but users are given a random word every session.

## Setup
The project uses the following:
- Rust
- Axum
- Bootstrap
- Nginx
- Docker
- Docker Compose

For additional information on project specifications, see the `Cargo.toml` file in `app`.

### Setting up the Server
In the `app/` directory, create a `.env` file that contains the following environment variables:
```
CORS_ALLOWED_ORIGINS="http://localhost http://127.0.0.1"
```

## Building
The project uses Docker. Ensure Docker and Docker Compose are installed before continuing.

To build, run `docker compose build`

## Running
To run the web API, run `docker compose up -d`, then go to http://localhost using your web browser.
