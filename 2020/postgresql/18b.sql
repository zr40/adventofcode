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
        with recursive calculation (remaining) as (
            select matches[2]

            union all

            select
                case
                    -- when debug(format(e'\nremaining: %s\nplus_matches: %s\nexpr_matches: %s\n', remaining, plus_matches, expr_matches), null) = '' then ''
                    when plus_matches is not null then plus_matches[1] || (plus_matches[3]::bigint + plus_matches[4]::bigint)::text || plus_matches[5]
                    when expr_matches[2] = '*' then (expr_matches[1]::bigint * expr_matches[3]::bigint)::text || expr_matches[4]
                    else expr_matches[1]
                end
            from
                calculation,
                regexp_match(remaining, '^((.*\*)?)(\d*)\+(\d*)(.*)$') as plus_matches,
                regexp_match(remaining, '^(\d+)([*]?)(\d*)(.*)$') as expr_matches
            where
                -- debug(remaining, remaining)
                remaining ~ '[*+]'
        )
        select
            -- debug(format(e'\ndebug\nremaining: %s\nvalue: %s', remaining, value), remaining::text)::bigint as value
            remaining::bigint as value
        from calculation
        where
            -- debug(remaining::text, remaining)
            remaining !~ '[+*]'
    ) as result
)
select sum(value)
from processed
where line !~ '\(';


rollback;
