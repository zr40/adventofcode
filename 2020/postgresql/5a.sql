begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null);
\copy input (line) from input/5


with passes as (
    select
        case when substr(line, 1, 1) = 'F' then 0 else 512 end +
        case when substr(line, 2, 1) = 'F' then 0 else 256 end +
        case when substr(line, 3, 1) = 'F' then 0 else 128 end +
        case when substr(line, 4, 1) = 'F' then 0 else 64 end +
        case when substr(line, 5, 1) = 'F' then 0 else 32 end +
        case when substr(line, 6, 1) = 'F' then 0 else 16 end +
        case when substr(line, 7, 1) = 'F' then 0 else 8 end +

        case when substr(line, 8, 1) = 'L' then 0 else 4 end +
        case when substr(line, 9, 1) = 'L' then 0 else 2 end +
        case when substr(line, 10, 1) = 'L' then 0 else 1 end as id

    from input
)
select max(id)
from passes;


rollback;
