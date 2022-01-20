FROM rustlang/rust:nightly as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/yrwsm-api /usr/local/bin/yrwsm-api
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["yrwsm-api"]