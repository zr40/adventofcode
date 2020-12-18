begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text, line_number integer primary key generated always as identity);
\copy input (line) from input/18


create function debug(input text, output text) returns text language plpgsql as $$
    begin
        raise notice '%', input;
        return output;
    end
$$;


with recursive processed (line_number, step, line, l, expr, value, r) as (
    select line_number, 0, '(' || regexp_replace(line, ' ', '', 'g') || ')', null, null, null::bigint, null
    from input

    union all

    select line_number, step + 1, matches[1] || result.value || matches[3], matches[1], matches[2], result.value, matches[3]
    from processed, regexp_match(line, '^(.*)\(([^()]*)\)(.*)$') as matches
    cross join lateral (
        with recursive calculation (value, remaining) as (
            select 0::bigint, matches[2]

            union all

            select
                -- case debug(format(e'\nvalue: %s\n1: %s\n2: %s\n3: %s', value, expr_matches[1], expr_matches[2], expr_matches[3]), null) = '' then ''
                case expr_matches[1]
                    when '' then expr_matches[2]::bigint
                    when '+' then value + expr_matches[2]::bigint
                    when '*' then value * expr_matches[2]::bigint
                end,
                expr_matches[3]
            from calculation, regexp_match(remaining, '^([+*]?)(\d*)(.*)$') as expr_matches
            where remaining != ''
        )
        select value
        from calculation
        where remaining = ''
    ) as result
)
select sum(value)
from processed
where line !~ '\(';


rollback;
