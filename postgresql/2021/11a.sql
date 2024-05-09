begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity primary key, value text not null);
\copy input (value) from input/11

create temporary table octopus (x integer, y integer, energy integer, flashing bool not null, flashed bool not null, primary key (x, y));
insert into octopus (x, y, energy, flashing, flashed)
select ord, line, energy::integer, false, false
from input, regexp_split_to_table(value, '') with ordinality as r(energy, ord);

create function step() returns integer language plpgsql as $$
begin
    -- increase energy level by 1 (and reset state)
    update octopus set energy = coalesce(energy, 0) + 1, flashing = false, flashed = false;

    -- find flashes
    update octopus set energy = null, flashing = true where energy > 9;

    -- increase energy level by adjacent flashes
    while exists(select from octopus where flashing) loop
        update octopus set energy = energy + flashes
        from (
            select this.x, this.y, count(*) as flashes
            from octopus as this
            inner join octopus as adjacent on this.x between adjacent.x - 1 and adjacent.x + 1 and this.y between adjacent.y - 1 and adjacent.y + 1
            where adjacent.flashing
            group by this.x, this.y
        ) as adjacent
        where octopus.x = adjacent.x and octopus.y = adjacent.y;

        update octopus set flashing = false, flashed = true where flashing;

        update octopus set energy = null, flashing = true where energy > 9;
    end loop;

    -- set energy to 0 after flash
    update octopus set energy = 0 where flashed;

    return count(*) from octopus where flashed;
end
$$;

select sum(step()) from generate_series(1, 100);

rollback;
