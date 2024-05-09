begin;
create schema aoc;
set search_path to aoc;

create table input (line text);
\copy input from input/1

select sum(line::integer) from input;

rollback;
