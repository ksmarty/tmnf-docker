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

ENV SERVER_NAME="TMNF Docker Server" \
    SERVER_PASS="P@ssw0rd123" \
    MYSQL_DATABASE="aseco" \
    MYSQL_USER="tmf" \
    MYSQL_PASSWORD="MYSQL_P4SS" \
    MYSQL_ROOT_PASSWORD="MYSQL_R00T_P4SS" \
    SERVER_PORT="2350" \
    P2P_PORT="3450" \
    RPC_PORT="5000" \
    ADMINS="" \
    AUTOSAVE="OFF" \
    RANDOM_MAP_ORDER="0"

LABEL maintainer="kyle@kyleschwartz.ca"
LABEL version="1.0"
LABEL description="TrackMania Nations Forever Server"

RUN apt-get update && \
    apt install -y software-properties-common gpg-agent && \
    add-apt-repository ppa:ondrej/php && \
    apt-get install -y php5.6 php5.6-xml php5.6-common php5.6-mysql unzip && \
    mkdir /tmnf

COPY --from=builder target/release/tmnf_setup /tmnf_setup

WORKDIR /tmnf

COPY zips/* ./

RUN unzip -q \*.zip && \
    rm *.zip && \
    cp xaseco/newinstall/*.xml xaseco && \
    cp xaseco/newinstall/*.php xaseco/includes

COPY configs/Custom.txt GameData/Tracks/MatchSettings/Nations
COPY configs/guestlist.txt GameData/Config

VOLUME /tmnf

CMD ["/tmnf_setup"]