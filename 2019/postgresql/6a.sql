begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/6

create temporary table orbits (object text primary key, orbiting text not null);

insert into orbits (orbiting, object)
select split_part(line, ')', 1), split_part(line, ')', 2) from input;

explain analyze with recursive orbit_count as (
    select orbiting
    from orbits

    union all

    select orbits.orbiting
    from orbit_count
    inner join orbits on orbit_count.orbiting = orbits.object
)
select count(*) from orbit_count;

rollback;
