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
), autocomplete (line, value, score) as (
    select line, value, 0::bigint
    from lines
    where final and value !~ '[\]}>)]'

    union all

    select
        line,
        r[1],
        score * 5 + case r[2]
            when '(' then 1
            when '[' then 2
            when '{' then 3
            when '<' then 4
            else 0
        end
    from autocomplete, regexp_match(value, '^(.*)(.)$') as r
    where value != ''
)

select percentile_disc(0.5) within group ( order by score ) from autocomplete
where value = ''
;

rollback;
