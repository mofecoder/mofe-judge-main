FROM mysql

EXPOSE 3306

COPY ./sql /docker-entrypoint-initdb.d
COPY my.cnf /etc/mysql/conf.d/mycnf

CMD ["mysqld"]