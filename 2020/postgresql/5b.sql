begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null);
\copy input (line) from input/5


with passes as (
    select
        case when substr(line, 1, 1) = 'F' then 0 else 64 end +
        case when substr(line, 2, 1) = 'F' then 0 else 32 end +
        case when substr(line, 3, 1) = 'F' then 0 else 16 end +
        case when substr(line, 4, 1) = 'F' then 0 else 8 end +
        case when substr(line, 5, 1) = 'F' then 0 else 4 end +
        case when substr(line, 6, 1) = 'F' then 0 else 2 end +
        case when substr(line, 7, 1) = 'F' then 0 else 1 end as r,

        case when substr(line, 8, 1) = 'L' then 0 else 4 end +
        case when substr(line, 9, 1) = 'L' then 0 else 2 end +
        case when substr(line, 10, 1) = 'L' then 0 else 1 end as c

    from input
), ids as (
    select r, c, r * 8 + c as id
    from passes
)
select myid
from generate_series(0, 1023) as myid
where not exists (select from ids where id = myid)
and exists (select from ids where id = myid + 1)
and exists (select from ids where id = myid - 1);


rollback;