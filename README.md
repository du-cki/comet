# Comet

Comet is a **blazingly fast** Content Delivery Network (CDN) node written in Rust. It's a basic project that was created as a learning exercise in Rust programming.

## Codebase Overview

The codebase is organized into several main files:

### main.rs

This is the main entry point for the application. It sets up the database connection, loads the application settings, creates the application routes, and starts the server.

### models.rs

This file contains the data structures used in the application. It includes the AppState struct, which holds the database pool and configuration settings, and several response structs used in the API endpoints.

### settings.rs

This file handles the loading of application settings from the `comet-config.toml` file and the environment. The settings include server binding address and port, password, file name length, file save path, fallback content type, and API endpoints.

### utils.rs

This file contains several utility functions used throughout the application. These include functions for checking if a file path exists, generating a unique file path, handling internal server errors, and parsing file names.

## Installation

Before you start, make sure you have Rust installed on your system. If you don't have it installed, you can download it from the official website: <https://www.rust-lang.org/tools/install>

1. **Clone the repository**:

    ```bash
    git clone https://github.com/du-cki/comet.git
    ```

2. **Navigate to the cloned repository**:

    ```bash
    cd comet
    ```

3. **Build the project**:

    ```bash
    cargo build
    ```

4. **Run the project**:

    ```bash
    cargo run
    ```

## Usage

Once you've installed Comet, you can start using it as a CDN node. Here's a basic guide on how to use it:

### Uploading a File

To upload a file, you need to make a POST request to the `/upload_file` endpoint. The request should include the file you want to upload and the password set in the configuration.

### Retrieving a File

To retrieve a file, make a GET request to the `/get_file/{filename}` endpoint, replacing `{filename}` with the name of the file you want to retrieve.

### Deleting a File

To delete a file, make a DELETE request to the `/delete_file/{filename}` endpoint, replacing `{filename}` with the name of the file you want to delete. This request should also include the password set in the configuration.

Please note that the actual endpoints may vary depending on your configuration settings.
Configuration

You can configure Comet by modifying the `comet-config.toml` file. Here are the available settings:

`bind_addr`: The IP address to bind the server to.
`bind_port`: The port to bind the server to.
`password`: The password required to upload and delete files.
`file_name_length`: The length of the generated file names.
`enforce_file_extensions`: Whether to enforce file extensions when generating file names.
`file_save_path`: The path where uploaded files will be saved.
`fallback_content_type`: The content type to use when the file's content type cannot be determined.
`endpoints.get_file`: The endpoint for retrieving files.
`endpoints.upload_file`: The endpoint for uploading files.
`endpoints.delete_file`: The endpoint for deleting files.

Please note that you should restart the server for the changes to take effect.

## Development

If you are interested in contributing to the development of Comet, here are some guidelines:

1. **Environment Setup**

   Make sure you have Rust installed on your system. If not, you can download it from the official website: <https://www.rust-lang.org/tools/install>

2. **Clone the Repository**

   Clone the repository to your local machine:

   ```bash
   git clone <https://github.com/du-cki/comet.git>

2. **Build the Project**

    Navigate to the cloned repository and build the project:

    ```bash
    cd comet
    cargo build
    ```

4. **Database Setup**

    Comet uses SQLite for its database. To set up the database, you'll need to perform an offline compilation with `SQLX_OFFLINE=true cargo run` to create the initial tables.

5. **Environment Variables**

    Create an `.env` file in the root of the project and set `DATABASE_URL` as `sqlite://data.db` for compile-time checks.

6. **Making Changes**

    If you make changes to the SQL queries, you'll need to regenerate the sqlx-data.json file for successful offline compilation. You can regenerate it with the sqlx CLI through cargo sqlx prepare.

7. **Testing**

    Make sure to test your changes before submitting a pull request. You can run the tests with:

    ```bash
    cargo test
    ```

8. **Submitting Changes**

    Once you've made your changes, push them to your fork and submit a pull request. Please provide a clear and detailed explanation of your changes in the pull request description.

Please note that this is a basic guide and might not cover all the details. If you have any questions or issues, feel free to open an issue on the GitHub repository.

### License

Comet is licensed under the MIT license.

### Contact

For more information, please visit the GitHub repository at <https://github.com/du-cki/comet>.
