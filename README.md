# Wishlist Web App

A web application for creating and managing personal wishlists with sharing capabilities.

## Project Structure

This is a monorepo containing both the frontend and backend:

- Root - Rust backend using Axum (Cargo.toml, src/)
- `frontend/` - Svelte frontend with TypeScript
- `SPEC.md` - Application specification
- `DATABASE.md` - Database schema specification

## Configuration

Configuration can be provided via environment variables or a TOML file. Environment variables will override any value set in the TOML file.

### Environment Variables

- `WISHLIST_DATABASE_URL` - PostgreSQL database connection string
  - Example: `postgresql://localhost/wishlist`

### TOML Configuration

Create a `wishlist.toml` file in the current working directory or in `$XDG_CONFIG_HOME/wishlist/`.

See `wishlist.toml.example` for the configuration format.

## Development

### Backend

```bash
cargo run
```

Run tests:

```bash
cargo test
```

Run migrations:

```bash
cargo run --bin migrate
```

This will run all pending migrations. You can also pass additional arguments like `down` or `status`:

```bash
cargo run --bin migrate -- down
cargo run --bin migrate -- status
```

Generate entities from database schema:

```bash
cargo run --bin generate-entity
```

Note: Make sure migrations have been run first so the database tables exist.

### Frontend

```bash
cd frontend
bun install
bun run dev
```

Run tests:

```bash
bun test
```

## Tech Stack

See [SPEC.md](SPEC.md) for the complete technical requirements.
