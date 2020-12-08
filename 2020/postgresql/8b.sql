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


with recursive state (clock, pc, acc, changed_instruction, finished) as (
    select 1, 1, 0, loc, false
    from instructions
    where instruction <> 'acc'

    union all

    select
        state.clock + 1,
        state.pc + case when (instruction = 'jmp' and changed_instruction <> pc) or (instruction = 'nop' and changed_instruction = pc) then instructions.value else 1 end,
        state.acc + case when instruction = 'acc' then instructions.value else 0 end,
        changed_instruction,
        instruction is null
    from state
    left join instructions on state.pc = instructions.loc
    where state.clock < (select max(loc) from instructions)
    and not finished
)
select acc, changed_instruction from state
where finished;


rollback;
