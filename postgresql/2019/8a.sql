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

select count(*) filter (where value = 0), count(*) filter (where value = 1) * count(*) filter (where value = 2) from pixels group by layer order by 1 limit 1;

rollback;
