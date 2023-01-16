FROM rust:1.61.0

WORKDIR /usr/src/media
COPY . .

RUN cargo install --path .

CMD ["media"]