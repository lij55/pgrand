-- create extension
create extension random_fdw;

-- create foreign data wrapper and foreign server'
create foreign data wrapper random_wrapper handler random_fdw_handler;

create server random_server foreign data wrapper random_wrapper;

-- create foreign table with options
create foreign table hello (
       c1 bigint,
       c2 int,
       c3 smallint
) server random_server options (total '100');