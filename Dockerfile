FROM rust:latest as build

ENV ENV="DEVELOPMENT"

RUN USER=root cargo new --bin build_app
WORKDIR /build_app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/rust_backend_app*
RUN cargo build --release

FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /build_app/target/release/rust-backend-app .

EXPOSE 3000
ENTRYPOINT [ "./rust-backend-app" ]
