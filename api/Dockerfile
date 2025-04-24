FROM rust:1.85

WORKDIR /usr/src/katchup/api
COPY ../api .

RUN cargo install --path .

CMD ["api"]

