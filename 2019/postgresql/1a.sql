begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/1

select sum((line::integer)/3-2) from input;

rollback;
