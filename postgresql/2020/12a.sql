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


with recursive ship (east, north, direction, step) as (
    select 0, 0, 90, 0

    union all

    select
        case action
            when 'E' then east + amount
            when 'W' then east - amount
            when 'F' then
                case direction
                    when 90 then east + amount
                    when 270 then east - amount
                    else east
                end
            else east
            end,
        case action
            when 'N' then north + amount
            when 'S' then north - amount
            when 'F' then
                case direction
                    when 0 then north + amount
                    when 180 then north - amount
                    else north
                end
            else north
            end,
        case action
            when 'R' then (direction + amount + 360) % 360
            when 'L' then (direction - amount + 360) % 360
            else direction
        end,
        step + 1
    from ship
    inner join actions on ship.step + 1 = actions.line_number
)
select abs(east) + abs(north)
from ship
where step = currval('input_line_number_seq');


rollback;
