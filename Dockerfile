FROM rust:latest

WORKDIR /usr/src/lcpae
COPY . .

RUN cargo install

CMD ["lcpae"]
