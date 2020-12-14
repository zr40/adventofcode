begin;
create schema aoc2;
set search_path to aoc2;
set jit to off;

create temporary table input (line text, line_number integer generated always as identity primary key);
\copy input (line) from input/14


with recursive parsed (pos, value, line_number) as (
    select
        nullif(split_part(split_part(line, 'mem[', 2), '] = ', 1), ''),
        split_part(line, ' = ', 2),
        line_number
    from input
), instructions (mask, pos, value, line_number) as (
    select case when parsed.pos is null then parsed.value end,
           parsed.pos,
           case when parsed.pos is not null then parsed.value::bigint end,
           parsed.line_number
    from parsed
    where parsed.line_number = 1

    union all

    select case when parsed.pos is null then parsed.value else instructions.mask end,
           parsed.pos,
           case when parsed.pos is not null then parsed.value::bigint end,
           parsed.line_number
    from instructions
             cross join lateral (select * from parsed where instructions.line_number + 1 = parsed.line_number) as parsed
), initial (mask, pos, value, line_number) as (
    select mask, pos::bigint::bit(36) | translate(mask, 'X', '0')::bit(36), value, line_number
    from instructions
), unnested as (
    select
        _.mask,
        _.pos,
        initial.value,
        initial.line_number
    from initial
    cross join lateral (
        with recursive permutations (mask, pos) as (
            select
                regexp_replace(initial.mask, 'X', new_bit::text),
                set_bit(initial.pos, strpos(initial.mask, 'X')-1, new_bit)
            from unnest(array[0,1]) as _(new_bit)

            union all

            select
                regexp_replace(permutations.mask, 'X', new_bit::text),
                set_bit(permutations.pos, strpos(permutations.mask, 'X')-1, new_bit)
            from permutations, unnest(array[0,1]) as _(new_bit)
            where permutations.mask ~ 'X'
        )
        select * from permutations
    ) as _
    where initial.pos is not null
    and _.mask !~ 'X'
), mem (pos, value, line_number) as (
    select pos::bigint | mask::bit(36)::bigint, value, line_number
    from unnested
    where pos is not null
), uniq_mem (value) as (
    select distinct on (pos) value::bigint, pos from mem order by pos, line_number desc
)
select sum(value) from uniq_mem;


rollback;
