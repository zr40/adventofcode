begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity, value text not null);
\copy input (value) from input/2

with lines as (
    select line, s[1] as dir, s[2]::int as distance
    from input, string_to_array(value, ' ') as s
    order by line
), instructions as (
    select
        line,
        case dir when 'up' then -distance when 'down' then distance else 0 end as depth,
        case dir when 'forward' then distance else 0 end as horizontal
    from lines order by line
), steps as (
    select
        sum(depth) over (order by line) as aim,
        depth,
        horizontal
    from instructions
    order by line

)
select sum(horizontal * aim) * sum(horizontal) from steps;

rollback;
