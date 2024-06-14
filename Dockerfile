FROM oven/bun:1 as frontend

WORKDIR /app/ui

COPY ui/package.json ui/bun.lockb ./
RUN bun install --frozen-lockfile

COPY ui/ .
RUN bun run build


FROM rust:1.76 as backend

WORKDIR /app

COPY . .
COPY --from=frontend /app/ui/dist /app/ui/dist

# TODO: store password on the database instead of in .env
# TODO: Also, remove the default password below. Putting it here to test.
ARG password="hiii"

ARG file_name_length="8"
ARG retain_uploaded_file_name="false"
ARG default_public="false"
ARG file_size_limit="0"
ARG enforce_file_extensions="true"
ARG fallback_content_type="application/octet-stream"
ARG RUST_LOG="RUST_LOG=none,comet=debug"

# These are concrete for all containers.
ENV APP_PORT=3000
ENV APP_BIND="[0, 0, 0, 0]"
ENV APP_FILE_SAVE_PATH="/media"

ENV APP_PASSWORD=${password}
ENV APP_FILE_NAME_LENGTH=${file_name_length}
ENV APP_RETAIN_UPLOADED_FILE_NAME=${retain_uploaded_file_name}
ENV APP_DEFAULT_PUBLIC=${default_public}
ENV APP_FILE_SIZE_LIMIT=${file_size_limit}
ENV APP_ENFORCE_FILE_EXTENSIONS=${enforce_file_extensions}
ENV APP_FALLBACK_CONTENT_TYPE=${fallback_content_type}

ENV RUST_LOG=${RUST_LOG}

ENV SQLX_OFFLINE=true

RUN cargo build --release

# TODO: Persist both ./data.db and ./media/ files in the container.
# TODO: Make ./data.db accessible from the outside
# TODO: Make the ability to export the ./media directory to the outside

# FROM debian:buster-slim

# WORKDIR /app

# COPY --from=backend /app/target/release/comet /app/comet
# COPY --from=backend /app/ui/dist /app/ui/dist

# EXPOSE 3000

# CMD ["/app/comet"]
