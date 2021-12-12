begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity primary key, value text not null);
\copy input (value) from input/10

with recursive lines (line, value, final) as (
    select line, value, false
    from input

    union all

    select
        line,
        replace(replace(replace(replace(value, '()', ''), '[]', ''), '<>', ''), '{}', ''),
        replace(replace(replace(replace(value, '()', ''), '[]', ''), '<>', ''), '{}', '') = value
    from lines
    where not final
)

select sum(
    case (regexp_match(value, '[\]}>)]'))[1]
    when ')' then 3
    when ']' then 57
    when '}' then 1197
    when '>' then 25137
end)
from lines
where final and value ~ '[\]}>)]';

rollback;
