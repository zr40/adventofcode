begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity, value text not null);
\copy input (value) from input/3

select gamma::bit(13)::integer * epsilon::bit(13)::integer from (
    select
        '0' || string_agg(case when ones > zeroes then '1' else '0' end, '') as gamma,
        '0' || string_agg(case when zeroes > ones then '1' else '0' end, '') as epsilon
    from (
        select ord, count(*) filter (where bit = '1') as ones, count(*) filter (where bit = '0') as zeroes
        from input, regexp_split_to_table(value, '') with ordinality as r(bit, ord)
        group by ord
        order by ord
    ) as _
) as _
;

rollback;
