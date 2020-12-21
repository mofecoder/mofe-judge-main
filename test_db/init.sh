#!/usr/bin/bash

docker \
    exec -i test_db_db_1 \
    sh -c 'cat sql/*.sql | mysql -uroot -proot'