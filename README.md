# Wishlist Web App

A web application for creating and managing personal wishlists with sharing capabilities.

## Project Structure

This is a monorepo containing both the frontend and backend:

- Root - Rust backend using Axum (Cargo.toml, src/)
- `frontend/` - Svelte frontend with TypeScript
- `SPEC.md` - Application specification
- `DATABASE.md` - Database schema specification

## Development

### Backend

```bash
cargo run
```

Run tests:

```bash
cargo test
```

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
