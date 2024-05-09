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

with recursive routes (path, current) as (
    select array[]::text[], 'start'

    union all

    select array_append(routes.path, routes.current), paths.b
    from routes
    inner join paths on routes.current = paths.a
    where (paths.b = upper(paths.b) or paths.b != all (routes.path)) and current <> 'end'
)
select count(*)
from routes
where current = 'end';

rollback;
