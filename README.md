## Comet

Comet is a 🚀 **blazingly fast** 🚀 **C**ontent **D**elivery **N**etwork node written in Rust.

## Features

TODO

## Installation

Before you start, make sure you have Rust installed on your system. If you don't have it installed, you can download it from the official website: <https://www.rust-lang.org/tools/install>

1. **Clone the repository & Navigate to it**:

   ```bash
   git clone https://github.com/du-cki/comet.git
   cd comet
   ```

2. **Build the app**:

   ```bash
   SQLX_OFFLINE=true cargo build --release
   ```

3. **Edit the `comet-config.toml` file and apply the necessary configurations.**

4. **Run the app**:

   ```bash
   cargo run --release
   ```

   <sup>Refer to the [wiki](/wiki) for ways to run the app indefinitely and pointers on exposing the app to the internet.</sup>

## Usage

<details>
<summary>Uploading a File</summary>

```bash
$ curl -X POST http://localhost:3000/upload \
     -H "Authorization: $AUTH_TOKEN" \
     -F file=@image.jpg

{
    "file": "DLRWjS_p",
    "file_url": "/media/DLRWjS_p.jpg",
    "file_size": 194668
}
```

</details>

<details>
<summary>Deleting a File</summary>

```bash
curl -X DELETE http://localhost:3000/delete/DLRWjS_p \
     -H "Authorization: $AUTH_TOKEN"

{
    "message": "Removed."
}
```

</details>

## Contributing

If you are interested in contributing to the development of Comet, here's how to setup an development enviroment:

### Development Environment Setup

1. **Building the Project**

   As comet uses [`SQLx`](https://github.com/launchbadge/sqlx) under the hood, you'll need to set up the database initially. To do so, you will need to perform an offline compilation. Build the project with the env variable `SQLX_OFFLINE` as `true`:

   ```bash
   SQLX_OFFLINE=true cargo build
   ```

2. **Setup Environment Variables**

   After building the project initially, you can create an `.env` file in the root of the project and set `DATABASE_URL` as `sqlite://data.db` for compile-time checks.

If you make changes to any SQL queries, you'll need to regenerate the `/sqlx-data.json` file for any further offline compilation. You can regenerate it with the [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) through `cargo sqlx prepare`.

### License

This project is licensed under the MIT license.
