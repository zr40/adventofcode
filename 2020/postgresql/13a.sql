begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text, line_number integer generated always as identity);
\copy input (line) from input/13


with time as (
    select line::integer as time
    from input
    where line_number = 1
), bus as (
    select id::integer
    from input, regexp_split_to_table(line, ',') as id
    where line_number = 2 and id <> 'x'
)
select id, (id - (time % id)) % id, (id - (time % id)) % id * id
from bus, time
order by 2
limit 1;


rollback;
