-- noinspection SqlNoDataSourceInspectionForFile

-- create extension
drop extension if exists pgrand;
create extension pgrand;

create table tmp(data text);

-- table will will leak

create table t1 (c1 text) using random;

insert into tmp select c1 from t1 limit 10000000;

-- fdw is fine
-- create foreign data wrapper and foreign server'
create foreign data wrapper random handler random_fdw_handler;

create server random_server foreign data wrapper random;

create foreign table f1 (c1 text) server random_server;

insert into tmp select c1 from f1 limit 10000000;