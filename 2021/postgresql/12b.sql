begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (value text not null);
\copy input (value) from input/12

create temporary table paths (a text not null, b text not null, primary key (a, b));

insert into paths (a, b)
select r[1], r[2]
from input, regexp_split_to_array(value, '-') as r;

insert into paths (a, b)
select b, a
from paths;

create temporary table caves (name text primary key, big boolean not null);
insert into caves (name, big)
select distinct on (a) a, a = upper(a)
from paths;

with recursive routes (path, current, double_used) as (
    select array[]::text[], 'start', false

    union all

    select array_append(routes.path, routes.current), paths.b, double_used or (
        paths.b = lower(paths.b) and cardinality(routes.path) - cardinality(array_remove(routes.path, paths.b)) = 1
    )
    from routes
    inner join paths on routes.current = paths.a
    where
        (paths.b = upper(paths.b) or cardinality(routes.path) - cardinality(array_remove(routes.path, paths.b)) < case when double_used then 1 else 2 end)
        and current <> 'end'
        and paths.b <> 'start'
)
select count(*)
from routes
where current = 'end';

rollback;
