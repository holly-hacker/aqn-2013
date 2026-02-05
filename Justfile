default:
    @just --list

create-network:
    docker network create sql2013-network || true

run-container:
    docker run --rm --name aqn2013-sql --network sql2013-network -e MYSQL_ROOT_PASSWORD=my-secret-pw -p 3306:3306 -d mysql:5.6

import-data backup_file database_name="aqn2013":
    docker run --rm --network sql2013-network -v {{backup_file}}:/backup.sql mysql:5.6 sh -c "mysql -h aqn2013-sql -u root -pmy-secret-pw -e 'CREATE DATABASE IF NOT EXISTS {{database_name}};' && mysql -h aqn2013-sql -u root -pmy-secret-pw -D {{database_name}} < /backup.sql"

sql-shell:
    docker run --rm -it --network sql2013-network mysql:5.6 mysql -h aqn2013-sql -u root -pmy-secret-pw

stop-container:
    docker stop aqn2013-sql
