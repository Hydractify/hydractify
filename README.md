# Introduction

This is a [Discord] application for experimenting ideas on and serve [our community](https://discord.com/invite/uBdXdE9) needs.

# Requirements

* [Rust]
* [PostgreSQL]

# Getting started

Firstly what you want to do is clone this repository: 
```sh
git clone https://github.com/Hydractify/hydractify.git
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

Now you can just run the application! If you're not familiar with [Rust], you can run it through your terminal with:
```sh
cargo run
```

# Modules

## Self Role

This module is configured via `[self_roles]` in `config.toml`, which has two fields:

1. `enabled`
   - Whether the module is enabled or not. The commands are still registered, maybe they shouldn't.
2. `channel_id`
   - The ID of the channel where the self role message should be deployed.

When enabled and the message is _deployed_ then when a [Discord] user interacts with the module they will have that certain role added to them or removed.

### Slash commands

To configure and manage the message with the self assignable roles you must use the `/selfrole` slash commands, these being:

1. `/selfrole show`
   - Lists the registered self assignable roles.
2. `/selfrole deploy`
   - Deploys the message with the self assignable roles, optionally receiving a message ID to edit an existing one.
3. `/selfrole cleanup`
   - Removes from the list all roles that have been deleted.
4. `/selfrole remove`
   - Removes a specific role from the self assignable role list.
5. `/selfrole add `
   - Adds a specific role into the self assignable role list.

## Starboard

This module is configured via `[starboard]` in `config.toml`, which has four fields:

1. `enabled`
   - Whether the module is enabled or not.
2. `channel`
   - The channel that starboard messages should go to.
3. `emojis`
   - An array containing the emotes that trigger a starboard message, these can be an UTF-8 character like `⭐` or a custom emote like `<a:a_kirbyStar:894087344909606912>`.
4. `threshold`
   - How many emotes are needed to trigger a starboard message.

# Special thanks

This implementation is heavily based off of [etternabot](https://github.com/kangalio/etternabot/), it was a great starting point for me to understand how [`poise`](https://github.com/serenity-rs/poise/) and [`serenity`](https://github.com/serenity-rs/serenity/) work!


[Discord]: https://discord.com/
[PostgreSQL]: https://www.postgresql.org/
[Rust]: https://www.rust-lang.org/
