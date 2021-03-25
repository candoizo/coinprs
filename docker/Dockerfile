# # FROM archlinux:base-devel as builder
# #
# # # RUN sed -i "s/#IgnorePkg.*=/IgnorePkg = filesystem/g" /etc/pacman.conf
# #
# # RUN pacman -Syuq --noconfirm --noprogress rustup which gcc clang && \
# # rustup install nightly
# #
# # WORKDIR /opt/build
# #
# # # broke at some point 😞
# # # # cache rust deps
# # # COPY Cargo.* ./
# # # RUN \
# # # mkdir -p src/ && \
# # # echo "fn main () -> () {}" > src/main.rs && \
# # # cargo build --release
# #
# # # copy rest of src
# # COPY . .
# # RUN \
# # cargo build --release && \
# # strip target/release/coinprs
#
# FROM alpine:3.13.1
#
# WORKDIR /opt/build
#
# RUN \
# # apk update && \
# apk add --no-cache rustup gcc clang libcurl libc-dev openssl-dev && \
# rustup-init -y && \
# source $HOME/.cargo/env && \
# rustup default nightly
# COPY . .
# RUN \
# source $HOME/.cargo/env && \
# cargo build --release && \
# strip target/release/coinprs
#
# CMD ["/opt/build/target/release/coinprs", "-c", "/opt/build/sample.config.yml"]
#
# # COPY --from=builder /opt/build/target/release/coinprs .
#






FROM rustlang/rust:nightly-slim as builder

WORKDIR /opt/build
RUN apt update && apt install -y pkg-config libssl-dev
# RUN \
# # apk update && \
# apk add --no-cache rustup gcc clang libcurl libc-dev openssl-dev && \
# rustup-init -y && \
# source $HOME/.cargo/env && \
# rustup default nightly
COPY . .
RUN \
# source $HOME/.cargo/env && \
cargo build --release && \
strip target/release/coinprs

# FROM scratch
FROM debian:buster-slim
RUN  apt update && apt install -y libssl-dev && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
# CMD ["myapp"]


WORKDIR /opt/build
COPY --from=builder /opt/build/target/release/coinprs .
COPY --from=builder /opt/build/sample.config.yml .

CMD ["./coinprs", "-c", "/opt/build/sample.config.yml"]
