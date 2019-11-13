FROM rust:1.39-buster AS build

RUN apt-get update \
 && apt-get install -y clang libclang-dev libsqlite3-dev llvm-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /dungeon-helper/

RUN mkdir -p /dungeon-helper/src/ \
 && echo 'fn main() {}' > /dungeon-helper/src/main.rs

ADD ./Cargo.toml ./Cargo.lock /dungeon-helper/

RUN cargo build --release

ADD ./src/ /dungeon-helper/src/

RUN touch /dungeon-helper/src/main.rs \
 && cargo build --release

FROM debian:buster

RUN apt-get update \
 && apt-get install -y sqlite3 libsqlite3-dev

COPY --from=build /dungeon-helper/target/release/dungeon_helper /opt/dungeon-helper/bin/dungeon_helper

ADD ./config/sql/* /opt/dungeon-helper/share/sql/

ADD ./config/bin/* /opt/dungeon-helper/bin/

CMD ["/opt/dungeon-helper/bin/entrypoint.sh"]
