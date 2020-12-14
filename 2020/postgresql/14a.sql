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
), instructions (mask_zeroes, mask_ones, pos, value, line_number) as (
    select
        case when parsed.pos is null then translate(parsed.value, 'X', '1')::bit(36) end,
        case when parsed.pos is null then translate(parsed.value, 'X', '0')::bit(36) end,
        parsed.pos,
        case when parsed.pos is not null then parsed.value::bigint::bit(36) else null::bit(36) end,
        parsed.line_number
    from parsed
    where parsed.line_number = 1

    union all

    select
        case when parsed.pos is null then translate(parsed.value, 'X', '1')::bit(36) else mask_zeroes end,
        case when parsed.pos is null then translate(parsed.value, 'X', '0')::bit(36) else mask_ones end,
        parsed.pos,
        case when parsed.pos is not null then parsed.value::bigint::bit(36) else null::bit(36) end,
        parsed.line_number
    from instructions
    cross join lateral (select * from parsed where instructions.line_number + 1 = parsed.line_number) as parsed
), mem (pos, value, line_number) as (
    select pos, value::bigint::bit(36) & mask_zeroes | mask_ones, line_number
    from instructions
    where pos is not null
), uniq_mem (value) as (
    select distinct on (pos) value::bigint from mem order by pos, line_number desc
)
select sum(value) from uniq_mem;


rollback;
