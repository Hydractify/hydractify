# **This branch is under development**

# Introduction

This is a [Discord] application for experimenting ideas on and serve [our community](https://discord.com/invite/uBdXdE9) needs.

# Requirements

* [Rust]
* [PostgreSQL]

# Getting started

Firstly what you want to do is clone this repository: 
```sh
git clone -b dev/rewrite https://github.com/Hydractify/hydractify.git`
```

From here you will configure the application with `config.toml`, you can use [`config.example.toml`](./config.example.toml) as a reference. Here you will set:

* The application's token (`token`);
* The database URL (`database_url`), i.e. `postgres://username:password@localhost/database_name`;
* [Module](#Modules) configuration.

Once that's done, you need to install `diesel_cli` for automatically generating typings and running our SQL to make sure all tables are configured. You can install it through cargo:
```sh
cargo install diesel_cli --no-default-features --features postgres
```
Once that's done, ensure it's binary in your `$PATH` then run:
```sh
diesel migration run
```

Now you can just run the application! If you're not familiar with rust, you can run it through your terminal with:
```sh
cargo run
```

## Modules

### Self Role

This module is configured via `[self_roles]` in `config.toml`, which has two fields:

1. `enabled`
    - Whether the module is enabled or not. The commands are still registered, maybe they shouldn't.
2. `channel_id`
    - The ID of the channel where the self role message should be deployed.

When enabled and the message is _deployed_ then when a [Discord] user interacts with the module they will have that certain role added to them or removed.

![Self Role deploy example](./docs/self_role_deploy.png)

> _More documentation is needed later, with screenshots when ran in the actual bot._

[Discord]: https://discord.com/
[PostgreSQL]: https://www.postgresql.org/
[Rust]: https://www.rust-lang.org/
