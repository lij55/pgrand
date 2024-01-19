-- noinspection SqlNoDataSourceInspectionForFile

-- create extension
create extension random_fdw;

-- create foreign data wrapper and foreign server'
create foreign data wrapper random_wrapper handler random_fdw_handler;

create server random_server foreign data wrapper random_wrapper;

-- create foreign table with options
create foreign table hello (
       c1 int2,
       c2 int4,
       c3 float,
       c4 decimal(8,2),
       t1 text,
       t2 char(2),
       a1 real[]
) server random_server options (total '20', seed '123456');
