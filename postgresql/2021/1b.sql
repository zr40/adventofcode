begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity, value integer not null);
\copy input (value) from input/1

select count(*) from (
    select
       value + lag(value) over w + lag(value, 2) over w >
       lag(value) over w + lag(value, 2) over w + lag(value, 3) over w
           as increased
    from input
    window w as (order by line)
) as _
where increased;

rollback;
