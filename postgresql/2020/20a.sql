begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text, line_number integer primary key generated always as identity);
\copy input (line) from input/20

create temporary table tiles (id integer primary key, contents bit(100) not null);
create temporary table tile_sides (id integer, side integer, border integer not null, border_flipped integer not null, primary key (id, side));
create index on tile_sides (border);
create index on tile_sides (border_flipped);

insert into tiles (id, contents)
select
    split_part(split_part(string_agg(line, '') filter (where line_number % 12 = 1), 'Tile ', 2), ':', 1)::integer,
    translate(string_agg(line, '' order by line_number) filter (where line_number % 12 > 1), '.#', '01')::bit(100) from input
group by line_number / 12;


--    normal           flipped       flipped (mirrored)
--      -->              <--               -->
--       0                4                 4
--  ^   +-+   |      |   +-+   ^       ^   +-+   |
--  | 3 |>| 1 |      | 5 |>| 7 |       | 7 |<| 5 |
--  |   +-+   v      v   +-+   |       |   +-+   v
--       2                6                 6
--      <--              -->               <--


create function border(
    contents bit(100),
    bit0 integer,
    bit1 integer,
    bit2 integer,
    bit3 integer,
    bit4 integer,
    bit5 integer,
    bit6 integer,
    bit7 integer,
    bit8 integer,
    bit9 integer
) returns integer language sql immutable as $$
    select
        get_bit(contents, bit0) +
        (get_bit(contents, bit1) << 1) +
        (get_bit(contents, bit2) << 2) +
        (get_bit(contents, bit3) << 3) +
        (get_bit(contents, bit4) << 4) +
        (get_bit(contents, bit5) << 5) +
        (get_bit(contents, bit6) << 6) +
        (get_bit(contents, bit7) << 7) +
        (get_bit(contents, bit8) << 8) +
        (get_bit(contents, bit9) << 9);
$$;
create function border_flipped(
    contents bit(100),
    bit9 integer,
    bit8 integer,
    bit7 integer,
    bit6 integer,
    bit5 integer,
    bit4 integer,
    bit3 integer,
    bit2 integer,
    bit1 integer,
    bit0 integer
) returns integer language sql immutable as $$
    select
        get_bit(contents, bit0) +
        (get_bit(contents, bit1) << 1) +
        (get_bit(contents, bit2) << 2) +
        (get_bit(contents, bit3) << 3) +
        (get_bit(contents, bit4) << 4) +
        (get_bit(contents, bit5) << 5) +
        (get_bit(contents, bit6) << 6) +
        (get_bit(contents, bit7) << 7) +
        (get_bit(contents, bit8) << 8) +
        (get_bit(contents, bit9) << 9);
$$;

insert into tile_sides (id, side, border, border_flipped)
select
    id,
    side,
    case side
        when 0 then border(contents, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
        when 1 then border(contents, 9, 19, 29, 39, 49, 59, 69, 79, 89, 99)
        when 2 then border(contents, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90)
        when 3 then border(contents, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0)

        when 4 then border(contents, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0)
        when 5 then border(contents, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90)
        when 6 then border(contents, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99)
        when 7 then border(contents, 99, 89, 79, 69, 59, 49, 39, 29, 19, 9)
    end,
    case side
        when 0 then border_flipped(contents, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
        when 1 then border_flipped(contents, 9, 19, 29, 39, 49, 59, 69, 79, 89, 99)
        when 2 then border_flipped(contents, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90)
        when 3 then border_flipped(contents, 90, 80, 70, 60, 50, 40, 30, 20, 10, 0)

        when 4 then border_flipped(contents, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0)
        when 5 then border_flipped(contents, 0, 10, 20, 30, 40, 50, 60, 70, 80, 90)
        when 6 then border_flipped(contents, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99)
        when 7 then border_flipped(contents, 99, 89, 79, 69, 59, 49, 39, 29, 19, 9)
    end
from tiles, generate_series(0, 7) as gs(side);


create temporary table state (
    x integer not null,
    y integer not null,
    tile integer not null,
    topside integer not null,
    primary key (x, y),
    unique (tile)
);
create index on state (tile, topside);

create function rotate(source integer, turns_right integer) returns integer immutable language sql as $$
    select case
        when source < 4 then (source + turns_right) % 4
        else (source + turns_right) % 4 + 4
    end;
$$;


create function fill_row(this_y integer, length integer) returns void language plpgsql as $$
    begin
        for this_x in 1..length-1 loop
            insert into state (x, y, tile, topside)
            select this_x, this_y, nexttile.id, rotate(nexttile.side, 1)
            from state
            inner join tile_sides as thistile on thistile.id = state.tile and thistile.side = rotate(state.topside, 1)
            inner join tile_sides as nexttile on thistile.border = nexttile.border_flipped and thistile.id <> nexttile.id
            where state.x = this_x - 1 and state.y = this_y;
        end loop;
    end
$$;

create function fill_field(length integer) returns void language plpgsql as $$
    begin
        perform fill_row(0, length);

        for this_y in 1..length-1 loop
            insert into state (x, y, tile, topside)
            select 0, this_y, nexttile.id, rotate(nexttile.side, 0)
            from state
            inner join tile_sides as thistile on thistile.id = state.tile and thistile.side = rotate(state.topside, 2)
            inner join tile_sides as nexttile on thistile.border = nexttile.border_flipped and thistile.id <> nexttile.id
            where state.x = 0 and state.y = this_y - 1;

            perform fill_row(this_y, length);
        end loop;
    end;
$$;

create function try_solve(size integer, tilecount integer) returns void language plpgsql as $$
    declare
        try_id integer;
        try_side integer;
    begin
        for try_id, try_side in with borders as (
            select (array_agg(id))[1] as id, (array_agg(side))[1] as sides, count(*)
            from tile_sides
            group by border
            having count(*) = 1
        ), corner as (
            select id, array_agg(sides) as orientations
            from borders
            group by id
            having sum(count) = 4
        )
        select
            id,
            unnest(orientations)
        from corner
        loop
            truncate state;

            insert into state (x, y, tile, topside) values (0, 0, try_id, try_side);

            perform fill_field(size);

            -- raise notice 'id % and orientation %: % tiles placed', try_id, try_side, (select count(*) from state);

            if (select count(*) = tilecount from state) then
                return;
            end if;

        end loop;
    end
$$;

select try_solve(sqrt(count(*))::integer, count(*)::integer) from tiles;

create aggregate product(bigint) (stype=bigint, sfunc=int8mul, initcond=1);

select product(tile)
from state
where
    (x = (select min(x) from state) or x = (select max(x) from state)) and
    (y = (select min(y) from state) or y = (select max(y) from state));


rollback;
