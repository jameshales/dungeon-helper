FROM centos:8

RUN dnf install -y sqlite-devel

ADD ./target/release/dungeon_helper /opt/dungeon-helper/bin/dungeon_helper

ADD ./config/sql/* /opt/dungeon-helper/share/sql/

ADD ./config/bin/* /opt/dungeon-helper/bin/

CMD ["/opt/dungeon-helper/bin/entrypoint.sh"]
