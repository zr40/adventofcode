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

create temporary table image (
    x integer not null,
    y integer not null,
    value boolean not null,
    primary key (x, y)
);

create function debug(t text, output anyelement) returns anyelement language plpgsql as $$
    begin
        raise notice '%', t;
        return output;
    end
$$;

create function create_image() returns void language sql as $$
    insert into image (x, y, value)
    select
        img_x + state.x * 8,
        img_y + state.y * 8,
        case
            when img_x > 7 or img_y > 7 then null
            else get_bit(tiles.contents,
                case state.topside
                    when 0 then img_y * 10 + img_x + 11
                    when 1 then img_x * 10 + (7 - img_y) + 11
                    when 2 then (7 - img_y) * 10 + (7 - img_x) + 11
                    when 3 then (7 - img_x) * 10 + img_y + 11

                    when 4 then img_y * 10 + (7 - img_x) + 11
                    when 5 then img_x * 10 + img_y + 11
                    when 6 then (7 - img_y) * 10 + img_x + 11
                    when 7 then (7 - img_x) * 10 + (7 - img_y) + 11
                end
            ) = 1
        end
    from state
    inner join tiles on state.tile = tiles.id
    cross join generate_series(0, 7) as gsx(img_x)
    cross join generate_series(0, 7) as gsy(img_y);
$$;

create function contains_sea_monster() returns boolean language sql as $$
    select exists (
        select

        from image as ref
        inner join image as tail on ref.x + 1 = tail.x and ref.y + 1 = tail.y and tail.value

        inner join image as bodyleft1 on ref.x + 4 = bodyleft1.x and ref.y + 1 = bodyleft1.y and bodyleft1.value
        inner join image as bodyleft2 on ref.x + 5 = bodyleft2.x and ref.y = bodyleft2.y and bodyleft2.value
        inner join image as bodyleft3 on ref.x + 6 = bodyleft3.x and ref.y = bodyleft3.y and bodyleft3.value
        inner join image as bodyleft4 on ref.x + 7 = bodyleft4.x and ref.y + 1 = bodyleft4.y and bodyleft4.value

        inner join image as bodyright1 on ref.x + 10 = bodyright1.x and ref.y + 1 = bodyright1.y and bodyright1.value
        inner join image as bodyright2 on ref.x + 11 = bodyright2.x and ref.y = bodyright2.y and bodyright2.value
        inner join image as bodyright3 on ref.x + 12 = bodyright3.x and ref.y = bodyright3.y and bodyright3.value
        inner join image as bodyright4 on ref.x + 13 = bodyright4.x and ref.y + 1 = bodyright4.y and bodyright4.value

        inner join image as head1 on ref.x + 16 = head1.x and ref.y + 1 = head1.y and head1.value
        inner join image as head2 on ref.x + 17 = head2.x and ref.y = head2.y and head2.value
        inner join image as head3 on ref.x + 18 = head3.x and ref.y = head3.y and head3.value
        inner join image as head4 on ref.x + 19 = head4.x and ref.y = head4.y and head4.value
        inner join image as head5 on ref.x + 18 = head5.x and ref.y - 1 = head5.y and head5.value

        where ref.value
    );
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

            if (select count(*) = tilecount from state) then
                -- raise notice 'id % and orientation %: % tiles placed', try_id, try_side, (select count(*) from state);

                truncate image;
                perform create_image();

                if contains_sea_monster() then
                    -- raise notice 'monster found';
                    return;
                end if;
            end if;

        end loop;
    end
$$;

select try_solve(sqrt(count(*))::integer, count(*)::integer) from tiles;

with monster(monstercount) as (
    select count(*) * 15

    from image as ref
    inner join image as tail on ref.x + 1 = tail.x and ref.y + 1 = tail.y and tail.value

    inner join image as bodyleft1 on ref.x + 4 = bodyleft1.x and ref.y + 1 = bodyleft1.y and bodyleft1.value
    inner join image as bodyleft2 on ref.x + 5 = bodyleft2.x and ref.y = bodyleft2.y and bodyleft2.value
    inner join image as bodyleft3 on ref.x + 6 = bodyleft3.x and ref.y = bodyleft3.y and bodyleft3.value
    inner join image as bodyleft4 on ref.x + 7 = bodyleft4.x and ref.y + 1 = bodyleft4.y and bodyleft4.value

    inner join image as bodyright1 on ref.x + 10 = bodyright1.x and ref.y + 1 = bodyright1.y and bodyright1.value
    inner join image as bodyright2 on ref.x + 11 = bodyright2.x and ref.y = bodyright2.y and bodyright2.value
    inner join image as bodyright3 on ref.x + 12 = bodyright3.x and ref.y = bodyright3.y and bodyright3.value
    inner join image as bodyright4 on ref.x + 13 = bodyright4.x and ref.y + 1 = bodyright4.y and bodyright4.value

    inner join image as head1 on ref.x + 16 = head1.x and ref.y + 1 = head1.y and head1.value
    inner join image as head2 on ref.x + 17 = head2.x and ref.y = head2.y and head2.value
    inner join image as head3 on ref.x + 18 = head3.x and ref.y = head3.y and head3.value
    inner join image as head4 on ref.x + 19 = head4.x and ref.y = head4.y and head4.value
    inner join image as head5 on ref.x + 18 = head5.x and ref.y - 1 = head5.y and head5.value

    where ref.value
), tile(tilecount) as (
    select count(*)
    from image
    where value
)
select tilecount - monstercount from monster, tile;


rollback;
