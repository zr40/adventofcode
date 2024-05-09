--begin;
drop schema if exists aoc cascade;
create schema aoc;
set search_path to aoc;
set jit to off;
reset client_min_messages;

create temporary table input (line text not null, line_number integer generated always as identity);
\copy input (line) from input/24

create temporary table directions (direction text primary key, dx integer not null, dy integer not null);
insert into directions (direction, dx, dy) values
    ('nw', -1, -1),
    ('ne', 1, -1),
    ('w', -2, 0),
    ('e', 2, 0),
    ('sw', -1, 1),
    ('se', 1, 1);

create unlogged table tiles (x integer not null, y integer not null, black boolean not null, primary key (x, y));

with destinations (x, y, line_number) as (
    select sum(directions.dx) as x, sum(directions.dy) as y, line_number
    from input
    cross join regexp_matches(line, 'nw|ne|e|se|sw|w', 'g') with ordinality as step(direction, pos)
    inner join directions on step.direction[1] = directions.direction
    group by line_number
), visited_tiles (x, y, visits) as (
    select x, y, count(*)
    from destinations
    group by x, y
)
insert into tiles
select x, y, visits % 2 = 1
from visited_tiles;

create function day() returns void language sql as $$
    insert into tiles (x, y, black)
    select distinct x+dx, y+dy, false from tiles
    cross join directions
    on conflict do nothing;

    with newstate (x, y, black) as (
        select thistile.x, thistile.y,
            case when thistile.black
                then count(*) filter (where neighbor.black) in (1, 2)
                else count(*) filter (where neighbor.black) = 2
            end
        from tiles as thistile
        cross join directions
        inner join tiles as neighbor on neighbor.x = thistile.x + directions.dx and neighbor.y = thistile.y + directions.dy
        group by thistile.x, thistile.y
    )
    update tiles
    set black = newstate.black
    from newstate
    where tiles.x = newstate.x and tiles.y = newstate.y;
$$;

do $$
    declare
        i integer;
    begin
        for i in 1..100 loop
            perform day();
            commit;
            raise notice '%', i;
        end loop;
    end
$$;

select count(*) from tiles where black;

--rollback;
drop schema aoc cascade;
