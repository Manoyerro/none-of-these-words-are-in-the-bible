FROM clux/muslrust:stable AS build

COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest AS run

COPY --from=build volume/target/x86_64-unknown-linux-musl/release/none-of-these-words-are-in-the-bible server
COPY --from=build volume/assets assets

RUN chmod +x server

ENTRYPOINT [ "./server" ]