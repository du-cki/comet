# Comet

A pretty basic CDN node, written in [Rust](https://rust-lang.org).

## Why?

This is a project I've been fiddling around to learn [rust](https://rust-lang.org). You shouldn't be using this in
production because It's pretty basic and has low security (single-key-authentication) for now, I plan to rectify this and
go for a more secure approach but that might prove tricky depending on the client you're using.

## Setup

The setup guide has now been moved to the [wiki](/wiki).

## Honorable mentions

- This service does not do any sort of media optimization/compression as if now, however this is planned for the future.
- You can save alot of bandwidth and make it more snappier if you have a caching layer, I.E cloudflare caching the images.

## Development

If you wish to work with this project, you'll have to do an offline compilation with `SQLX_OFFLINE=true cargo run`,
to create the initial tables, after which you can create an `.env` file to put `DATABASE_URL` as `sqlite://data.db`
to get compile time checks.

If you've made changes to the SQL queries, you'll have to regenerate the `sqlx-data.json` file if you'd wish to do an
successful offline compilation again. You can regenerate it with the [`sqlx cli`](https://github.com/launchbadge/sqlx)
through `cargo sqlx prepare`

## License

This project is following the [MIT license](/blob/master/LICENSE).
