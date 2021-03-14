# FROM mysql
FROM mysql@sha256:870892ea5cc8c623b389717c2eedd58248c82a8569d7601ede62a63d527641bd

EXPOSE 3306

COPY ./sql /docker-entrypoint-initdb.d
COPY my.cnf /etc/mysql/conf.d/mycnf

CMD ["mysqld"]