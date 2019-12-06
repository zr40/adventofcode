begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/6

create temporary table orbits (object text primary key, orbiting text not null);

insert into orbits (orbiting, object)
select split_part(line, ')', 1), split_part(line, ')', 2) from input;

with recursive orbit_count as (
    select object, orbiting as next_orbit, 0 as orbit_count
    from orbits

    union all

    select orbit_count.object, orbits.orbiting, orbit_count.orbit_count + 1
    from orbit_count
    left join orbits on orbit_count.next_orbit = orbits.object
    where orbit_count.next_orbit is not null
)
select sum(orbit_count) from orbit_count
where next_orbit is null;

rollback;
