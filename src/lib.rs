//! Monotax is an income and tax management tool for personal use.
//!
//! It is not a reliable tool. It's a toy project to learn Rust.
//!
//! The central concept of Monotax is an income. An income is a record of money received by a person.

pub mod app;
pub mod cli;
pub mod config;
pub mod domain;
pub mod filter;
pub mod income;
pub mod infra;
pub mod init;
pub mod integration;
pub mod payment;
pub mod report;
pub mod time;
