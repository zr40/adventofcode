begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null, line_number integer not null generated always as identity);
\copy input (line) from input/8
create temporary table instructions (instruction text not null, value integer not null, loc integer primary key);
insert into instructions (instruction, value, loc)
select split_part(line, ' ', 1), split_part(line, ' ', 2)::integer, line_number
from input;


with recursive state (clock, pc, acc) as (
    select 1, 1, 0

    union all

    select
        state.clock + 1,
        state.pc + case when instruction = 'jmp' then instructions.value else 1 end,
        state.acc + case when instruction = 'acc' then instructions.value else 0 end
    from state
    inner join instructions on state.pc = instructions.loc
    where state.clock < (select max(loc) from instructions)
)
select acc from state
where exists (
    select from state as prev_state
    where state.pc = prev_state.pc
    and state.clock > prev_state.clock
)
limit 1;


rollback;
