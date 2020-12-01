begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (value integer not null);
create index on input (value);
\copy input from input/1


select a.value * b.value * c.value

from input as a
inner join input as b on a.ctid > b.ctid
inner join input as c on b.ctid > c.ctid

where 2020 - a.value >= b.value
and 2020 - a.value - b.value = c.value;


rollback;
