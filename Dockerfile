FROM rustlang/rust:nightly-buster-slim as cargo-build
RUN apt update
RUN apt install -y default-libmysqlclient-dev pkg-config
WORKDIR /usr/src/rcms
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt update
RUN apt install -y default-libmysqlclient-dev pkg-config
WORKDIR /usr/src/rcms
COPY --from=cargo-build /usr/src/rcms/target/release/rust-cms /usr/bin/rcms
COPY templates ./templates
COPY migrations ./migrations
COPY wait-for-it.sh .

ENV APP_PRODUCTION=true

# Defaulted just in case someone wants to use mysqld.sock as their socket.
# This file doesn't even exist in the fs so I don't know why this would be defaulted here?
ENV MYSQL_UNIX_PORT = /var/lib/mysql/mysqld.sock

RUN ln -s /var/lib/mysql/mysqld.sock ${SOCKET_PATH}

CMD [ "rcms" ]
