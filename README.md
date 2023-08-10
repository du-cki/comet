# # Comet

Comet is a **blazingly fast** Content Delivery Network (CDN) node written in Rust.

## Installation

Before you start, make sure you have Rust installed on your system. If you don't have it installed, you can download it from the official website: <https://www.rust-lang.org/tools/install>

1. **Clone the repository & Navigate to it**:

    ```bash
    git clone https://github.com/du-cki/comet.git
    cd comet
    ```

3. **Build the project**:

    ```bash
    SQLX_OFFLINE=true cargo build --release
    ```

4. **Edit the `comet-config.toml` file and apply the necessary configurations.**

5. **Run the project**:

    ```bash
    cargo run --release
    ```
    
    <sup>Please refer to the wiki for other ways to running & exposing this project.</sup>

## Usage

Once you've installed comet, you can start using it as a CDN node. Here's a basic guide on how to use it:

### Uploading a File

To upload a file, you need to make a POST request to the `/upload` endpoint with the file (as FormData) and the password (set in the `comet-config.toml` file) in the `Authorisation` header.

### Retrieving a File

To retrieve a file, make a GET request to the `/media/:media_id` endpoint, replacing `:media_id` with the name of the file you want to retrieve.

### Deleting a File

To delete a file, make a DELETE request to the `/delete/:media_id` endpoint, replacing `:media_id` with the relevant file name and along with the password set in the Authorisation header.

## Configuration

You can configure Comet by modifying the `comet-config.example.toml` file. Here are the available settings 
(don't forget to rename the file into `comet-config.toml`):

`bind_addr`: The IP address to bind the server to. <br />
`bind_port`: The port to bind the server to. <br />
`password`: The password required to access authenticated routes (i.e. upload route). <br />
`file_name_length`: The length of the randomly generated file names. <br />
`file_save_path`: The path on disk where uploaded files will be saved onto. <br />
`file_size_limit`: The file upload limit, if you pass in `0`, this will be disabled. <br /> 
`enforce_file_extensions`: Whether to enforce file extensions when requesting for the file. <br />
`fallback_content_type`: The content type to use when the uploaded file's content type cannot be determined. <br />
`endpoints.get_file`: The endpoint for retrieving files. <br />
`endpoints.upload_file`: The endpoint for uploading files. <br />
`endpoints.delete_file`: The endpoint for deleting files. <br />

<sup>Please note that you should restart the server for the changes to take effect. </sup>

## Contributing

If you are interested in contributing to the development of Comet, here's how to setup an development enviroment:

### Dev Environment Setup

Make sure you have Rust installed on your system. If not, you can download it from the official website: <https://www.rust-lang.org/tools/install>

1. **Clone the Repository & Navigate to the project**

   Clone the repository to your local machine:

   ```bash
   git clone https://github.com/du-cki/comet.git
   cd comet
   ```

2. **Build the Project**

    As comet uses SQLite as its database as choice, you'll need to set up the database. To do so, you will need to perform
    an offline compilation. Build the project with the env variable `SQLX_OFFLINE` as `true`:

    ```bash
    SQLX_OFFLINE=true cargo build
    ```

3. **Environment Variables**

    After building the project initially, you can create an `.env` file in the root of the project and set `DATABASE_URL` as `sqlite://data.db` for compile-time checks.

If you make changes to any SQL queries, you'll need to regenerate the sqlx-data.json file for a successful offline compilation. You can regenerate it with the [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) through `cargo sqlx prepare`.

Once you've made your changes, please provide a clear and detailed explanation of your changes and open a pull request.

### ### License

comet is licensed under the MIT license.
