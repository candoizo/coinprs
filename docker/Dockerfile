FROM archlinux:base-devel as builder

RUN sed -i "s/#IgnorePkg.*=/IgnorePkg = filesystem/g" /etc/pacman.conf

RUN pacman -Syuq --noconfirm --noprogress rustup which gcc clang && \
rustup install nightly

WORKDIR /opt/build

# cache rust deps
COPY Cargo.* ./
RUN \
mkdir -p src/ && \
echo "fn main () -> () {}" > src/main.rs && \
cargo build --release

# copy rest of src
COPY . .
RUN \
cargo build --release && \
strip target/release/coinprs

FROM alpine:latest
COPY --from=builder /target/release/coinprs .
