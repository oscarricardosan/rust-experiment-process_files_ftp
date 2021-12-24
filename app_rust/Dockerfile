FROM rust:1.56

WORKDIR /usr/src/myapp

COPY . .

RUN curl -O https://www.openssl.org/source/old/0.9.x/openssl-0.9.8zf.tar.gz  && \
    tar xf openssl-0.9.8zf.tar.gz  && \
    cd openssl-0.9.8zf  && \
    export CC=...  && \
    ./Configure --prefix=... linux-x86_64 -fPIC  && \
    make -j$(nproc)  && \
    make install

RUN cargo install --path .

CMD ["myapp"]
