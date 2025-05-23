# Database Setup

1. Install sqlx CLI

```
cargo install sqlx-cli
```

2. Create migrations files and add the SQL commands

```
cargo sqlx migrate add <migration_name>
```

## Running Migrations

1. Run migrations

```
cargo sqlx migrate run
```

2. Run Prepare

```
cargo sqlx prepare
```
