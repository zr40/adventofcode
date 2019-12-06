begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/6

create temporary table orbits (object text primary key, orbiting text not null);

insert into orbits (orbiting, object)
select split_part(line, ')', 1), split_part(line, ')', 2) from input;

with recursive transfer_count as (
    select 'YOU' as object, -2 as transfer_count, array['YOU'] as seen

    union all

    select
        case when orbits.orbiting = transfer_count.object then orbits.object else orbits.orbiting end,
        transfer_count.transfer_count + 1,
        array_append(seen, case when orbits.orbiting = transfer_count.object then orbits.object else orbits.orbiting end)
    from orbits
    inner join transfer_count on orbits.orbiting = transfer_count.object or orbits.object = transfer_count.object
    where not seen @> array[
        case when orbits.orbiting = transfer_count.object then orbits.object else orbits.orbiting end
    ]
)
select transfer_count.transfer_count
from transfer_count
where object = 'SAN'
limit 1
;

rollback;
