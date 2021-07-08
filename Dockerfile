FROM rustlang/rust:nightly-buster-slim as cargo-build
RUN apt update
RUN apt install -y default-libmysqlclient-dev
WORKDIR /usr/src/rcms
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt update
RUN apt install -y default-libmysqlclient-dev
WORKDIR /usr/src/rcms
COPY --from=cargo-build /usr/src/rcms/target/release/rust-cms /usr/bin/rcms
COPY templates ./templates
COPY migrations ./migrations
COPY wait-for-it.sh .

ENV APP_PRODUCTION=true

CMD [ "rcms" ]
