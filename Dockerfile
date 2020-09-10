FROM rust:1.46 as builder

RUN apt-get update
RUN apt-get install -y clang-7

WORKDIR /lib
ADD . ./

RUN cargo test