FROM rust:slim AS build

COPY . .

RUN cargo build --release

FROM debian:stable-slim AS run

COPY --from=build target/release/none-of-these-words-are-in-the-bible server
COPY --from=build assets assets

RUN chmod +x server

ENTRYPOINT [ "./server" ]