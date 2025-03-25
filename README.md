# Boring Database Engine (BDE)

> A type-safe SQL toolkit for PHP built with Rust

## About

Boring Database Engine (BDE) is a code generator that transforms your SQL queries into type-safe PHP functions. Inspired
by the Go-based SQLC project, BDE brings the same productivity and safety benefits to the PHP ecosystem.

The "boring" philosophy is intentional - database interactions should be predictable, reliable, and free from unexpected
behavior. BDE focuses on generating straightforward, maintainable code without unnecessary complexity.

## Features

- **Type-safe SQL**: Generate PHP functions with proper type signatures from your SQL queries
- **Compile-time validation**: Catch SQL errors before runtime
- **Performance**: Rust-powered parsing and code generation
- **Minimal dependencies**: Clean, straightforward PHP output with no runtime dependencies
- **MySQL support**: First-class support for MySQL (additional databases planned)

## Getting Started

```bash
# Initialize a new project
bde init

# Generate code from your SQL files
bde generate
```

## Example

Define your SQL schema:

```sql
CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(255) NOT NULL UNIQUE,
    email      VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP    NOT NULL DEFAULT NOW()
);
```

Write your queries:

```sql
-- name: GetUserByID :one
SELECT *
FROM users
WHERE id = $1;

-- name: ListUsers :many
SELECT *
FROM users
ORDER BY created_at DESC;

-- name: CreateUser :one
INSERT INTO users (username, email)
VALUES ($1, $2) RETURNING *;
```

BDE will generate type-safe PHP code:

```php
function getUserByID(int $id): User 
{
    // Implementation generated by BDE
}

function listUsers(): array 
{
    // Implementation generated by BDE
}

function createUser(string $username, string $email): User
{
    // Implementation generated by BDE
}
```

## Status

BDE is currently in alpha. The core functionality works, but APIs may change as we work toward a stable release.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

MIT

## Author

Peter Paravinja