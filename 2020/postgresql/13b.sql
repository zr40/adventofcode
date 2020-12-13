begin;
create schema aoc2;
set search_path to aoc2;
set jit to off;

create temporary table input (line text, line_number integer generated always as identity);
\copy input (line) from input/13


create function combine_interval(current bigint, start bigint, other_id bigint, other_pos bigint) returns table (num bigint, min bigint, max bigint) language sql as $$
    with recursive seq as (
        select start::bigint as pos

        union all

        select seq.pos + current
        from seq
    ), minmax as (
        select pos
        from seq
        where (pos + other_pos) % other_id = 0
        limit 2
    )
    select max(pos) - min(pos), min(pos), max(pos) from minmax;
$$;

with recursive bus as (
    select id::bigint, pos-1 as pos
    from input, regexp_split_to_table(line, ',') with ordinality as r(id, pos)
    where line_number = 2 and id <> 'x'
), intervals as (
    select 1::bigint as i, -1::bigint as pos, 0::bigint as answer, 0::bigint as start

    union all

    select num, bus.pos, c.min, c.max
    from intervals
    cross join lateral (select id, pos from bus where bus.pos > intervals.pos order by bus.pos limit 1) as bus
    cross join lateral combine_interval(i, start, bus.id, bus.pos) as c
)
select max(answer) from intervals;


rollback;
