begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (value integer not null);
\copy input from input/1


select a.value * b.value

from input as a
inner join input as b on a.ctid > b.ctid

where 2020 - a.value = b.value;


rollback;
