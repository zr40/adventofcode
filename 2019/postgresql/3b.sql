begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/3

create temporary table grid (
    x integer not null,
    y integer not null,
    line_number integer not null,
    distance integer not null
);
create index on grid (x, y);

set jit = off;

insert into grid (x, y, line_number, distance)
select start_x + segment_position * direction_x as x, start_y + segment_position * direction_y as y, line_number, accumulated_distance + segment_position as distance from (
select generate_series(1, distance) as segment_position, direction_x, direction_y, start_x, start_y, line_number, accumulated_distance from (
select
    direction_x,
    direction_y,
    distance,
    coalesce(sum(direction_x * distance) over (partition by line_number rows between unbounded preceding and 1 preceding), 0) as start_x,
    coalesce(sum(direction_y * distance) over (partition by line_number rows between unbounded preceding and 1 preceding), 0) as start_y,
    coalesce(sum(distance) over (partition by line_number rows between unbounded preceding and 1 preceding), 0) as accumulated_distance,
    line_number
from (
select case(direction) when 'R' then 1 when 'L' then -1 else 0 end as direction_x, case(direction) when 'D' then 1 when 'U' then -1 else 0 end as direction_y, distance, line_number from (
select substring(instruction for 1) as direction, substring(instruction from 2)::integer as distance, line_number from (
select regexp_split_to_table(line, ',') as instruction, row_number() over () as line_number from input
) as _
) as _
) as _
) as _
) as _;

select min(a.distance + b.distance)
from grid as a
inner join grid as b on a.x = b.x and a.y = b.y
where a.line_number = 1 and b.line_number = 2;

rollback;
