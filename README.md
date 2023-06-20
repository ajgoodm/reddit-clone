# Reddit-esque backend API

Loosely inspired by the recent
[Reddit kerfuffle](https://abcnews.go.com/US/wireStory/reddit-blackout-explained-thousands-subreddits-protesting-party-app-100151669),
this is intended to be a backend API
for a Reddit-like app.

**Motivation:** this is a toy project to explore the
[Axum web framework](https://docs.rs/axum/latest/axum/), which is a framework
for building performant web servers in Rust. This project implements a simple
CRUD interface over a Postgres database. Database migrations and mappings between
tables and Rust structs are accomplished using the
[Diesel crate](https://diesel.rs), while a connection pool is maintained using
the adorably named [BB8 crate](https://crates.io/crates/bb8).
