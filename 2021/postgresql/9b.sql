begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity primary key, value text not null);
\copy input (value) from input/9

create temporary table points (x integer not null, y integer not null, height integer not null, primary key (x, y));
create index on points (y, x);

create aggregate product(bigint) (sfunc = int8mul, stype = int8);

insert into points
select ord, line, digit::integer
from input, regexp_split_to_table(value, '') with ordinality as r(digit, ord);

with recursive basin_map (x, y, height, basin) as (
    select x, y, height, row_number() over (order by x, y)
    from points
    where not exists (
        select from points as adjacent
        where adjacent.height <= points.height
        and ((
            adjacent.x = points.x and (adjacent.y = points.y + 1 or adjacent.y = points.y - 1)
        ) or (
            adjacent.y = points.y and (adjacent.x = points.x + 1 or adjacent.x = points.x - 1)
        ))
    )

    union

    select adjacent.x, adjacent.y, adjacent.height, basin
    from basin_map
    inner join points as adjacent on (
        adjacent.x = basin_map.x and (adjacent.y = basin_map.y + 1 or adjacent.y = basin_map.y - 1)
    ) or (
        adjacent.y = basin_map.y and (adjacent.x = basin_map.x + 1 or adjacent.x = basin_map.x - 1)
    )
    where adjacent.height < 9 and basin_map.height < adjacent.height
)
select product(count) from (
    select count(*) from basin_map
    group by basin
    order by count desc
    limit 3
) as _
;

rollback;
