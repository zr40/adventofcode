begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/5

create temporary table memory (position integer primary key, value integer not null);
insert into memory (value, position) select value::integer, position - 1 from input, regexp_split_to_table(input.line, ',') with ordinality as r(value, position);

create function output(t text) returns void language plpgsql as $$
    begin raise notice '%', t; end
$$;

create function debug(t text) returns void language sql as $$
    -- select output(t);
$$;

create function intcode_vm(pc integer) returns void volatile language sql as $$
    select 0/0;
$$;

create function read_instruction(pc integer, out opcode integer, out mode_1 integer, out mode_2 integer, out mode_3 integer) stable language sql as $$
    select debug(format('reading instruction at pc %s: %s', pc, value))
    from memory
    where position = pc;

    select value % 100, value / 100 % 10, value / 1000 % 10, value / 10000
    from memory
    where position = pc;
$$;

create function read(read_position integer, mode integer, silent boolean = false) returns integer stable language sql as $$
    select debug(format('reading %s in mode %s: %s', read_position, mode, value))
    from memory
    where position = read_position and not silent;

    select case mode when 1 then value when 0 then read(value, 1, silent) else null end
    from memory
    where position = read_position;
$$;

create function opcode_1(pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns void volatile language sql as $$
    select debug(format('writing %s to %s', read(pc + 1, mode_1) + read(pc + 2, mode_2), read(pc + 3, 1)));

    update memory
    set value = read(pc + 1, mode_1, true) + read(pc + 2, mode_2, true)
    where position = read(pc + 3, 1, true);
    
    select intcode_vm(pc + 4);
$$;

create function opcode_2(pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns void volatile language sql as $$
    select debug(format('writing %s to %s', read(pc + 1, mode_1) * read(pc + 2, mode_2), read(pc + 3, 1)));

    update memory
    set value = read(pc + 1, mode_1, true) * read(pc + 2, mode_2, true)
    where position = read(pc + 3, 1, true);

    select intcode_vm(pc + 4);
$$;

create function opcode_3(pc integer, mode_1 integer) returns void volatile language sql as $$
    select debug(format('writing 1 to %s', read(pc + 1, 1)));

    update memory
    set value = 1
    where position = read(pc + 1, 1, true);

    select intcode_vm(pc + 2);
$$;

create function opcode_4(pc integer, mode_1 integer) returns void volatile language sql as $$
    select output(format('output: %s', read(pc + 1, mode_1)));

    select intcode_vm(pc + 2);
$$;

create or replace function intcode_vm(pc integer) returns void volatile language sql as $$
    select case opcode
        when 1 then opcode_1(pc, mode_1, mode_2, mode_3)
        when 2 then opcode_2(pc, mode_1, mode_2, mode_3)
        when 3 then opcode_3(pc, mode_1)
        when 4 then opcode_4(pc, mode_1)
        when 99 then null
        else null
        end
    from read_instruction(pc), debug(format('pc %s instruction %s modes %s %s %s', pc, opcode, mode_1, mode_2, mode_3));
$$;

select intcode_vm(0);

rollback;
