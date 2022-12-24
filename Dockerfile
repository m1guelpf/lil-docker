FROM rust:1.62-buster

# Grab the dependencies and compile them as they dont change much
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock

RUN mkdir /app/src
RUN echo 'fn main() { println!("Hello World!"); }' > /app/src/main.rs

WORKDIR /app
RUN cargo build --release --target-dir=/tmp/lil_docker
RUN cargo clean -p lil_docker --release --target-dir=/tmp/lil_docker

# Grab the real code
ADD . /app

RUN sed -i -e 's/\r$//' /app/docker.sh

ENTRYPOINT ["/app/docker.sh"]
