begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text, line_number integer primary key generated always as identity);
\copy input (line) from input/17


create temporary table space (x int not null, y int not null, z int not null, primary key (x, y, z));
create temporary table space2 (x int not null, y int not null, z int not null, primary key (x, y, z));

insert into space (x, y, z)
select line_number-1, y-1, 0
from input,
regexp_split_to_table(line, '') with ordinality as point(contents, y)
where contents = '#';

create function iterate() returns void language sql as $$
    with candidates (x, y, z) as (
        select distinct x + dx, y + dy, z + dz
        from space, generate_series(-1, 1) as dx, generate_series(-1, 1) as dy, generate_series(-1, 1) as dz
    ), new_space (x, y, z) as (
        select candidates.* from candidates
        cross join lateral (
            select count(*)
            from space
            where candidates.x between space.x - 1 and space.x + 1
            and candidates.y between space.y - 1 and space.y + 1
            and candidates.z between space.z - 1 and space.z + 1
            and not (candidates.x = space.x and candidates.y = space.y and candidates.z = space.z)
        ) as neighbors
        left join space on candidates.x = space.x and candidates.y = space.y and candidates.z = space.z
        where neighbors.count = 3 or (neighbors.count = 2 and space.x is not null)
    ), del as (delete from space)
    insert into space2 table new_space;

    truncate space;
    insert into space table space2;
    truncate space2;
$$;

select iterate() from generate_series(1, 6) \g /dev/null
select count(*) from space;


rollback;
