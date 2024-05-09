begin;
create schema aoc;
set search_path to aoc;
set jit to off;

\set preamble_size 25

create temporary table input (num bigint not null, line_number integer not null generated always as identity primary key);
\copy input (num) from input/9
analyze input;

explain analyze
with recursive invalid_number as (
    select num
    from input
    where not exists (
        select
        from input as a
        inner join input as b on a.line_number < b.line_number
        where a.num + b.num = input.num
        and a.line_number between input.line_number - :preamble_size -1 and input.line_number - 1
        and b.line_number between input.line_number - :preamble_size -1 and input.line_number - 1
    )
    and line_number > :preamble_size
), ranges (start, stop, sum) as (
    select 1, 0, bigint '0'
    
    union all

    select
        case when ranges.sum > invalid_number.num then ranges.start + 1 else ranges.start end,
        case when ranges.sum < invalid_number.num then ranges.stop + 1 else ranges.stop end,
        (
            select sum(num)::bigint
            from input
            where line_number
                between case when ranges.sum > invalid_number.num then ranges.start + 1 else ranges.start end
                and case when ranges.sum < invalid_number.num then ranges.stop + 1 else ranges.stop end
        )
    from ranges, invalid_number
    where ranges.sum <> invalid_number.num
)
select min(input.num) + max(input.num)
from ranges
cross join invalid_number
inner join input on input.line_number between ranges.start and ranges.stop
where ranges.sum = invalid_number.num;


rollback;
