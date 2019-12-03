begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/3

create temporary table grid (x integer not null, y integer not null, crossing boolean not null, primary key (x, y));

set jit = off;

insert into grid (x, y, crossing)
select x, y, min(line_number) <> max(line_number) from (
select start_x + segment_position * direction_x as x, start_y + segment_position * direction_y as y, line_number from (
select generate_series(1, distance) as segment_position, direction_x, direction_y, start_x, start_y, line_number from (
select
    direction_x,
    direction_y,
    distance,
    coalesce(sum(direction_x * distance) over (partition by line_number rows between unbounded preceding and 1 preceding), 0) as start_x,
    coalesce(sum(direction_y * distance) over (partition by line_number rows between unbounded preceding and 1 preceding), 0) as start_y,
    line_number
from (
select case(direction) when 'R' then 1 when 'L' then -1 else 0 end as direction_x, case(direction) when 'D' then 1 when 'U' then -1 else 0 end as direction_y, distance, line_number from (
select substring(instruction for 1) as direction, substring(instruction from 2)::integer as distance, line_number from (
select regexp_split_to_table(line, ',') as instruction, row_number() over () as line_number from input
) as _
) as _
) as _
) as _
) as _
) as _ group by x, y
;

/*
select string_agg(case (select crossing from grid where grid.x = _x and grid.y = _y) when true then 'X' when false then '+' else '.' end, '' order by _x)
from
    generate_series((select min(x) from grid), (select max(x) from grid)) as _x,
    generate_series((select min(y) from grid), (select max(y) from grid)) as _y
group by _y
order by _y;
*/

/*with recursive _ as (
    select 0 as x, 0 as y, false as crossing, 0 as distance
    union all
    select grid.x, grid.y, grid.crossing, distance + 1
    from _
    inner join grid on (grid.x = _.x and grid.y in (_.y + 1, _.y - 1)) or (grid.y = _.y and grid.x in (_.x + 1, _.x - 1))
)
select distance from _ where crossing limit 1;
*/

select min(abs(x) + abs(y)) from grid where crossing;

rollback;
