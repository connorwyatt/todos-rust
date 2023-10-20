# todos-rust

## Running

First, you need Postgres running. There is a docker compose file in the root that will get you set up. If you set it up yourself, you might have to update some of the configuration in the project.

Next, you will need to install `sqlx-cli`:

```
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

Now you can run the migrations:

```
sqlx migrate run
```

Now you can run the application.
