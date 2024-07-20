# Use SQLX instead of rusqlite

Date: `2024-07-20`
Status: `Accepted`

## Context and Problem Statement

New versions of Monotax requires reliable database upgrade mechanisms to prevent data loss. The current implementation uses custom upgrade engine based on rusqlite. Supporting the engine is time-consuming and error-prone.

## Decision

Monotax will use SQLX instead of rusqlite.

SQLX has a mechanism to automatically upgrade the database schema.
Additional benefits of SQLX:

- Compile-time SQL query validation
- Type-safe query results
- Popular and well-maintained library

## Alternatives

### Maintain the current implementation

The upgrade engine does not carry complex tasks now, so the maintenance may not be a big issue. But keeping a complex component in the project is not a design goal for Monotax.

### Use Diesel ORM

Diesel is a popular ORM for Rust. It has a built-in migration engine. It also supports working in a synchronous environment. Monotax is currently a synchronous application, so Diesel would be a good fit.

The development plan of Monotax includes creation of REST API. Most popular libraries use async model to handle request. Using Diesel would require briding between async runtime and sync commands. While this coexistence is possible, it adds more development effort.

## Consequences

- Monotax will migrate to async model using Tokion runtime.
- The rusqilite will be excluded from the project.
- The upgrade engine will be replaces via SQLX.

## Changelog

- 2024-07-20: Initial version
