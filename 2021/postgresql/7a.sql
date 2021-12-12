begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (value text not null);
\copy input (value) from input/7

with positions as (
    select pos::integer from input, string_to_table(value, ',') as p(pos)
)
select sum(abs(pos-target)), target
from positions
cross join generate_series((select min(pos) from positions), (select max(pos) from positions)) as gs(target)
group by target
order by sum
limit 1
;


rollback;
