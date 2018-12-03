begin;
create schema aoc;
set search_path to aoc;

create table input (line text);
\copy input from input/1

create table numbers_seen (i bigint primary key);

set statement_timeout to '5s';

insert into numbers_seen
select sum(line::integer) over (rows unbounded preceding) from input, generate_series(1, 10000);

-- unique violation contains the result

rollback;
