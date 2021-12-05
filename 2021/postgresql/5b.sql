begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity, value text not null);
\copy input (value) from input/5

create temporary table lines (x1 integer not null, y1 integer not null, x2 integer not null, y2 integer not null);

with raw as (
    select regexp_split_to_array(value, '(,| -> )') arr
    from input
)
insert into lines (x1, y1, x2, y2)
select
    arr[1]::integer as x1,
    arr[2]::integer as y1,
    arr[3]::integer as x2,
    arr[4]::integer as y2
from raw;
create index on lines (x1);
create index on lines (x2);
create index on lines (y1);
create index on lines (y2);
analyze lines;

with bounds as (
    select
        min(least(x1, x2)) as min_x,
        max(greatest(x1, x2)) as max_x,
        min(least(y1, y2)) as min_y,
        max(greatest(y1, y2)) as max_y
    from lines
), positions as (
    select x, y, count(*) --, array_agg(lines), array_agg(x1), array_agg(x2), array_agg(y1), array_agg(y2)
    from bounds
    cross join generate_series(min_x, max_x) as x
    cross join generate_series(min_y, max_y) as y
    inner join lines on
        (x between symmetric x1 and x2 and y = y1 and y = y2)
        or
        (y between symmetric y1 and y2 and x = x1 and x = x2)
        or
        (x between symmetric x1 and x2 and y between symmetric y1 and y2 and (
            (x1 < x2 and y1 < y2 and x - y = x1 - y1)
            or
            (x1 > x2 and y1 < y2 and x + y = x1 + y1)
            or
            (x1 < x2 and y1 > y2 and x + y = x1 + y1)
            or
            (x1 > x2 and y1 > y2 and x - y = x2 - y2)
        ))
    group by y, x
    order by y, x
)
select count(*)
from positions
where count > 1
;

rollback;
