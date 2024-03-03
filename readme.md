# Introduction
pgrand is a Postgres extension in rust that helps to create random test data quickly. It provides an FDW and a table AM(in progress) that could generate random test data.

# Build
## Prepare tools

Install rust tool chain if needed:
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Install pgrx:
```bash
cargo install --locked cargo-pgrx --version 0.11.3
```

Then install your favourate Postgres distribution as usual.

## Prepare the environment
```bash
git clone https://github.com/lij55/pgrand.git
cd pgrand
cargo pgrx init --pg15=/path/to/pg15/pg_config
```
Please change the `/path/to/pg15/pg_config` to the real path of pg_config on your host.

## Test(TBD), run and ship
Once the pgrx init is done successfully, you can start it by:
```bash
cargo pgrx run -r
```
The pgrx will show a psql console and you create and play the extension
```sql
create extension pgrand;
```

To create an installable package, run
```bash
cargo pgrx package
```

# Examples

## Simple example of Random FDW
```sql
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
```

## Random test data for pgvector
```sql
-- create extension vector;
-- set random.array_length = 10;
create table test (id serial, data vector);
create foreign table foreign_vec(data real[]) server random_server;

insert into test(data) select data::vecotr(10) from foreign_vec;
```

# Features and others

Why pgrand?
- In-Database
- Very fast because of AVX
- Control random output through variable(length, range etc.)
- Support PG version: 15, 16 (more will be added later)
- Support most common datatype (text, number, array, geo, json, etc.)


Known limitition:
- The generated number is not evenly distributed.
- Doesn't support UDT(for example, PostGIS data type)

TODO:
- Test, test, test
- Table AM besides FDW
- More data types if possible
- and others

Performance:

1. compared with generate_series
```sql
pgrand=# create foreign table test(c1 int) server random_server options(total '100000000');
CREATE FOREIGN TABLE
pgrand=# \timing
Timing is on.
pgrand=# select avg(c1) from test;
avg
------------------------
 0.57655918000000000000
(1 row)

Time: 6574.341 ms (00:06.574)

pgrand=# select avg(random()) from generate_series(1,100000000);
avg
--------------------
 0.4999935385792818
(1 row)

Time: 10285.914 ms (00:10.286)

```

2. for 1,000,000 vector of 2048-dims
```sql
drop table if exists test;
drop foreign table if exists foreign_vec;
set random.array_length = 2048;
create table test (data vector);
create foreign table foreign_vec(data real[]) server random_server options(total '1000000');

pgrand=# insert into test(data) select data::vector(2048) from foreign_vec;
INSERT 0 1000000
Time: 62911.406 ms (01:02.911)

```
