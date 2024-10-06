# Monotax

Monotax is a simple utility to help with tax reporting.

It generates reports from account statements.

It can prepare import files for awesome Taxer service.

## Next steps

- [x] Simplify income types. Possibly get rid of `DescribedIncome` trait.
- [x] Add application configuration holding Taxer metadata and tax rates.
- [x] Use all date filtering options from CLI.
- [x] Generate quarterly reports in CSV format.

## Ideas

- Implement GUI for the application.
- Implement interactive CLI.

## Development plans

0.2.0:
- [ ] Mark payments as done.
- [ ] Use storage (SQLite) mode by default.

Using storage by default leads to a few significant changes.

There won't be a separate mode to work with input CSV files.
You still can analyze CSV files on the fly, but you need to import incomes into the storage to preserve history and mark payments as done. 
## Building

### Development database

Building the project requires the database to be set up and present.
The database URL is expected to be stored in a `DATABASE_URL` environment variable. 

1. Install the `slqx-cli` tool via `cargo install sqlx-cli`.
2. Create the `.env` file by copying the `.env.example`.
3. Correct the value of the `DATABASE_URL` if necessary.
4. Initialize development database via `sqlx database setup`.


## Cross-compilation

This project can be cross-compiled for the Raspberry Pi (64-bit).
Please check the build directory for more information.
