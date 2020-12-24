begin;
create schema aoc;
set search_path to aoc;
set jit to off;
reset client_min_messages;

create temporary table input (line text not null, line_number integer generated always as identity);
\copy input (line) from input/24

--create temporary table steps (direction text not null, line_number integer not null, pos integer not null, primary key (line_number, pos));

create temporary table directions (direction text primary key, dx integer not null, dy integer not null);
insert into directions (direction, dx, dy) values
    ('nw', -1, -1),
    ('ne', 1, -1),
    ('w', -2, 0),
    ('e', 2, 0),
    ('sw', -1, 1),
    ('se', 1, 1);

with destinations (x, y, line_number) as (
    select sum(directions.dx) as x, sum(directions.dy) as y, line_number
    from input
    cross join regexp_matches(line, 'nw|ne|e|se|sw|w', 'g') with ordinality as step(direction, pos)
    inner join directions on step.direction[1] = directions.direction
    group by line_number
), tiles (x, y, visits) as (
    select x, y, count(*)
    from destinations
    group by x, y
)
select count(*)
from tiles
where visits % 2 = 1
;

rollback;
