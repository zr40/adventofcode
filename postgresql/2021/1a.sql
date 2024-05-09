begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity, value integer not null);
\copy input (value) from input/1

select count(*) from (
    select value > lag(value) over (order by line) as increased
    from input
) as _ where increased;

rollback;
