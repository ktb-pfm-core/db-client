in this directory demonstrates how to connect to CockroachDB with the [sqlx](https://crates.io/crates/sqlx).

## Prerequisites

You must have rust and Cargo installed on your local machine, and a running CockroachDB cluster.

## Step 1. Set the `DATABASE_URL` environment variable

Set the `DATABASE_URL` environment variable to the connection URL of your CockroachDB cluster.

```shell
export DATABASE_URL="postgresql://<username>:<password>@<hostname>:<port>/<name>"
```

Where:

- `<username>` is the SQL user on the CockroachDB cluster.
- `<password>` is the password for the SQL user.
- `<hostname>` is the hostname of the CockroachDB cluster.
- `<port>` is the port number on which CockroachDB is running on the host.
- `<name>` is the database name.
