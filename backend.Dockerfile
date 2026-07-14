FROM rust:1.94-alpine AS backend

WORKDIR /app

COPY --exclude=app/ . .

ENV SQLX_OFFLINE=true

EXPOSE 3000
RUN cargo build --release

CMD ["./target/release/comet"]