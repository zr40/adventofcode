begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/6

create temporary table orbits (object text primary key, orbiting text not null);

insert into orbits (orbiting, object)
select split_part(line, ')', 1), split_part(line, ')', 2) from input;

with recursive orbit_count as (
    select 'COM' as object, 0 as steps

    union all

    select orbits.object, steps + 1
    from orbit_count
    inner join orbits on orbit_count.object = orbits.orbiting
)
select sum(steps) from orbit_count;

rollback;
