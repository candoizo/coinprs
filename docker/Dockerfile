FROM archlinux:base-devel

RUN sed -i "s/#IgnorePkg.*=/IgnorePkg = filesystem/g" /etc/pacman.conf

RUN pacman -Syuq --noconfirm --noprogress rustup which gcc clang && \
rustup install nightly

WORKDIR /opt/build

# cache rust deps for build
COPY Cargo.* ./
RUN \
mkdir -p src/ && \
touch src/main.rs && \
cargo build --release

# copy rest of src
COPY . .
RUN \
cargo build --release
# RUN \
# wasm-pack build && \
# wasm-pack pack && \
# wasm-pack test --firefox --headless
