begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (action text, line_number integer generated always as identity);
\copy input (action) from input/12
create temporary table actions (action text not null, amount integer not null, line_number integer primary key);
insert into actions (action, amount, line_number)
select substring(action, 1, 1), substring(action, 2)::integer, line_number
from input;


with recursive ship (east, north, wp_east, wp_north, step) as (
    select 0, 0, 10, 1, 0

    union all

    select
        case action
            when 'F' then east + wp_east * amount
            else east
            end,
        case action
            when 'F' then north + wp_north * amount
            else north
            end,

        case action
            when 'E' then wp_east + amount
            when 'W' then wp_east - amount
            when 'L' then
                case amount
                    when 90 then -wp_north
                    when 180 then -wp_east
                    when 270 then wp_north
                    end
            when 'R' then
                case amount
                    when 90 then wp_north
                    when 180 then -wp_east
                    when 270 then -wp_north
                    end
            else wp_east
            end,
        case action
            when 'N' then wp_north + amount
            when 'S' then wp_north - amount
            when 'L' then
                case amount
                    when 90 then wp_east
                    when 180 then -wp_north
                    when 270 then -wp_east
                    end
            when 'R' then
                case amount
                    when 90 then -wp_east
                    when 180 then -wp_north
                    when 270 then wp_east
                    end
            else wp_north
            end,
        step + 1
    from ship
    inner join actions on ship.step + 1 = actions.line_number
)
select abs(east) + abs(north)
from ship
where step = currval('input_line_number_seq');


rollback;
