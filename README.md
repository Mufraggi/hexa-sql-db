# Migration Repository

# Setup

Commencer pars installer le cli de [sqlx](https://crates.io/crates/sqlx-cli) 

Refer a cette [exemple](https://github.com/launchbadge/sqlx/blob/master/examples/postgres/todos/README.md) 

```rust
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

run postgres

```docker
docker run --name postgresql-container -p 5432:5432 -e POSTGRES_PASSWORD=somePassword -d postgres
```

# Create Db

```rust
export DATABASE_URL="postgres://postgres:somePassword@localhost:5432/postgres"
```

```docker
$ sqlx db create
```

# Run migration

```docker
sqlx migrate run
```