FROM rust:1.51.0 as build-env

WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build-env /app/target/release/default-service /
CMD ["./default-service"]