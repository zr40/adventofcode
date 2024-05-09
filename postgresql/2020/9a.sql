begin;
create schema aoc;
set search_path to aoc;
set jit to off;

\set preamble_size 25

create temporary table input (num bigint not null, line_number integer not null generated always as identity primary key);
\copy input (num) from input/9


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
and line_number > :preamble_size;


rollback;
