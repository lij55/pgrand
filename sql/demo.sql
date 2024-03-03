-- noinspection SqlNoDataSourceInspectionForFile

-- create extension
create extension pgrand;

-- create foreign data wrapper and foreign server'
create foreign data wrapper random handler random_fdw_handler;
create server random_server foreign data wrapper random;

-- set extension variable to control random output
set random.array_length = 10;
set random.min_text_length = 8;
set random.max_text_length = 20;

-- create foreign table with 20 rows
create foreign table hello (
       c1 int2,
       c2 int4,
       c3 float,
       c4 decimal(8,2),
       a1 real[],
       t1 text,
       t2 char(2),
       j1 json,
       i1 inet,
       p1 point,
       b1 box
) server random_server options (total '20');

-- change total row number to 100
ALTER foreign table hello OPTIONS (set total '100');