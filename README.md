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

