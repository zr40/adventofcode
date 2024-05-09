begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity primary key, value text not null);
\copy input (value) from input/9

create temporary table points (x integer not null, y integer not null, height integer not null, primary key (x, y));
create index on points (y, x);

insert into points
select ord, line, digit::integer
from input, regexp_split_to_table(value, '') with ordinality as r(digit, ord);

select sum(height + 1)
from points
where not exists (
    select from points as adjacent
    where adjacent.height <= points.height
    and ((
        adjacent.x = points.x and (adjacent.y = points.y + 1 or adjacent.y = points.y - 1)
    ) or (
        adjacent.y = points.y and (adjacent.x = points.x + 1 or adjacent.x = points.x - 1)
    ))
);

rollback;
