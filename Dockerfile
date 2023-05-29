FROM rust:1.69.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8002

WORKDIR /app
COPY . .

RUN rustup default nightly

RUN cargo build

CMD ["cargo", "run"]
