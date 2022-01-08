FROM debian:buster-slim
RUN apt-get update
RUN apt install build-essential zlib1g-dev wget -y
    
RUN cd /usr/local/src/ && \
    wget https://www.openssl.org/source/old/0.9.x/openssl-0.9.8zg.tar.gz && \
    tar -xf openssl-0.9.8zg.tar.gz && \
    cd openssl-0.9.8zg && \
    ./config --prefix=/usr/local/ssl --openssldir=/usr/local/ssl shared zlib && \
    make && \
    make test && \
    make install
    
RUN apt install curl -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y 
    
ENV PATH="/root/.cargo/bin:${PATH}"


RUN rm -rf /var/lib/apt/lists/*
COPY . /home/process_files_in_ftp

WORKDIR /home/process_files_in_ftp

#CMD ["cargo", "run start"]
ENTRYPOINT ["tail", "-f", "/dev/null"]
