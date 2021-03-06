# Root

![Javscript CI](https://github.com/foxfriends/root/workflows/Javscript%20CI/badge.svg)  
![Rust CI](https://github.com/foxfriends/root/workflows/Rust%20CI/badge.svg)
[![dependency status](https://deps.rs/repo/github/foxfriends/root/status.svg)](https://deps.rs/repo/github/foxfriends/root)  
![Database CI](https://github.com/foxfriends/root/workflows/Database%20CI/badge.svg)

This is a digital port of the board game "Root" by Cole Wehrle, with art by Kyle Ferrin.

> Work in progress.

The rules are implemented according to the [Law of Root, Fifth Printing](./law-of-root.pdf) (July 13th, 2020).

The intention is to eventually support all expansions, though some parts may come later.
The planned order of implementation is as follows:
- [ ] All 8 factions, 9 vagabonds, autumn map, standard deck.
- [ ] Exiles and Partisans deck.
- [ ] Alternate maps (winter, mountain, lake).
- [ ] Rootbotics.

## Development

This is a [NodeJS][] (14) and [Rust][] (nightly) project, so both must be installed.
[PostgreSQL][] (13) is also a dependency for the database.

[NodeJS]: https://nodejs.org/en/
[Rust]: https://rustup.rs/
[PostgreSQL]: https://www.postgresql.org/

Though the server is written in Rust, the goal is to build the game logic using
[Lumber][]. Lumber is very work in progress, so things are likely to change often.

[Lumber]: https://github.com/foxfriends/lumber

Before starting development, you must first configure the environment and database.
Copy the `sample.env` file to `.env`, and configure the variables there as necessary.

Ensure [PostgreSQL][] is installed, and that you can connect to it. The initial database
setup can be performed using the `npm run db:setup` script. From there, use the [SQLx CLI][]
(install as `cargo install sqlx-cli`) to manage the database:

```sh
sqlx database create
sqlx migrate run
```

[SQLx CLI]: https://github.com/launchbadge/sqlx/tree/master/sqlx-cli

To compile the client code, use `npm`. We are using [Rollup][] for bundling,
and [Svelte][] as the framework.

[Rollup]: https://rollupjs.org/
[Svelte]: https://svelte.dev/

```sh
npm install # install dependencies
npm run dev # run builder
```

To compile and start the server, use Cargo. Dependencies are installed automatically with Cargo.

```sh
cargo run
# To run and print the logs:
RUST_LOG=root=debug cargo run
# If you have installed cargo-watch, watch just the Rust project:
cargo watch -x run -w src -w game -w Cargo.toml -w Cargo.lock
```

With both of those running, the app should be available on `localhost:3000`.
