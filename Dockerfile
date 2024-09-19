# Use the latest Debian base image
FROM debian:latest
WORKDIR /root

# Install dependencies for Rust, Isolate, and compilers/interpreters
RUN apt-get update && \
    apt-get install -y curl build-essential gcc g++ python3 python3-pip make git libcap-dev 
    
# Download and install Rust (which includes cargo) using the official Rust installer
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
    

# Set the environment path to include Rust and Cargo
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get install -y pkg-config libcap-dev libsystemd-dev asciidoc 
#Install isolate (sandboxing tool)
RUN git clone https://github.com/ioi/isolate.git 

RUN cd isolate && make isolate && make install

WORKDIR /code

CMD ["cargo", "run"]


