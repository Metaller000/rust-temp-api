ARG IMAGE=run
FROM ubuntu:24.04 as deb
WORKDIR /wrk
RUN apt update -qq && apt install -y -qq curl build-essential> /dev/null 
RUN curl https://sh.rustup.rs > rustup-init.sh \
    && sh rustup-init.sh -y \
    && cp $HOME/.cargo/bin/* /usr/local/bin/

COPY . . 
RUN cargo install cargo-deb 
RUN cargo deb
    
FROM rust:alpine as build
RUN apk add --no-cache curl musl-dev 
RUN curl https://sh.rustup.rs > rustup-init.sh \
    && sh rustup-init.sh -y 
    
COPY  . /app
WORKDIR /app
RUN /usr/local/cargo/bin/cargo build --bins --release

FROM scratch AS run
COPY --from=build /app/target/release/test-api /
COPY --from=build /app/cert.pem  /
COPY --from=build /app/key.pem  /

ENV HOME /root
ENV USER root
ENV SSL_CERT_DIR=/etc/ssl/certs/

ENTRYPOINT ["./test-api"]

FROM ${IMAGE}