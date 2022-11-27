############## 
# Setup
############## 

FROM rust:1.65 AS builder

COPY setup/ ./

RUN cargo build --release

############## 
# Final Image
############## 

FROM ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive

EXPOSE 2350/tcp
EXPOSE 2350/udp
EXPOSE 3450/tcp
EXPOSE 3450/udp
EXPOSE 5000/tcp
EXPOSE 5000/udp
EXPOSE 8002/tcp

LABEL maintainer="kyle@kyleschwartz.ca"
LABEL version="1.0"
LABEL description="TrackMania Nations Forever Server"

RUN apt-get update && \
    apt install -y software-properties-common gpg-agent && \
    add-apt-repository ppa:ondrej/php && \
    apt-get install -y php5.6 php5.6-xml php5.6-common php5.6-mysql unzip && \
    mkdir /tmnf

WORKDIR /tmnf

COPY --from=builder target/release/tmnf_setup ./tmnf_setup
COPY zips/* ./

RUN unzip -q \*.zip && \
    rm *.zip && \
    cp xaseco/newinstall/*.xml xaseco && \
    cp xaseco/newinstall/*.php xaseco/includes

CMD ["./tmnf_setup"]