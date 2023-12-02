FROM rust:1-slim-bookworm as builder

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /data

COPY --from=builder /app/target/release/question-marks-reply-bot-rs /app/question-marks-reply-bot-rs

VOLUME ["/data"]

ENV QUESTION_MARKS_REPLY_BOT_RS_DATA_DIR=/data

ENTRYPOINT ["/app/question-marks-reply-bot-rs"]