begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/8

create temporary table pixels (x integer not null, y integer not null, layer integer not null, value integer not null, primary key (layer, x, y));

insert into pixels (x, y, layer, value)
select
    (ordinality - 1) % 25,
    (ordinality - 1) / 25 % 6,
    (ordinality - 1) / 25 / 6,
    value::integer
from input, regexp_split_to_table(line, '') with ordinality as value;

select string_agg(case visible_pixel when 0 then ' ' else '#' end, '' order by x) from (
    select (array_agg(value order by layer) filter (where value <> 2))[1] as visible_pixel, x, y
    from pixels
    group by x, y
    order by x, y
) as _
group by y
order by y;

rollback;
