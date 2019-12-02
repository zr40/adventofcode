begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/2

create temporary table memory (position integer primary key, value integer not null);

create function intcode_vm(pc integer) returns void volatile language sql as $$
    select 0/0;
$$;

create function read_memory(read_position integer) returns integer stable language sql as $$
    select value from memory where position = read_position;
$$;
create function read_pointer(read_position integer) returns integer stable language sql as $$
    select mem_value.value from memory as mem_value inner join memory as mem_pointer on mem_pointer.value = mem_value.position where mem_pointer.position = read_position;
$$;

create function opcode_1(pc integer) returns void volatile language sql as $$
    update memory set value = read_pointer(pc + 1) + read_pointer(pc + 2) where position = read_memory(pc + 3);
    select intcode_vm(pc + 4);
$$;

create function opcode_2(pc integer) returns void volatile language sql as $$
    update memory set value = read_pointer(pc + 1) * read_pointer(pc + 2) where position = read_memory(pc + 3);
    select intcode_vm(pc + 4);
$$;

create function hcf() returns void volatile language sql as $$
    select 0/0;
$$;

create or replace function intcode_vm(pc integer) returns void volatile language sql as $$
    select case read_memory(pc)
        when 1 then opcode_1(pc)
        when 2 then opcode_2(pc)
        when 99 then null
        else hcf()
        end;
$$;

create function find_answer(noun integer, verb integer) returns boolean volatile language sql as $$
    truncate memory;
    insert into memory (value, position) select value::integer, position - 1 from input, regexp_split_to_table(input.line, ',') with ordinality as r(value, position);
    update memory set value = noun where position = 1;
    update memory set value = verb where position = 2;

    select intcode_vm(0);

    select read_memory(0) = 19690720;
$$;

select 100 * noun + verb from generate_series(0,99) as noun, generate_series(0,99) as verb where find_answer(noun, verb) limit 1;

rollback;
