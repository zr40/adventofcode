begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/7

create temporary table program (position integer primary key, value integer not null);
insert into program (value, position) select value::integer, position - 1 from input, regexp_split_to_table(input.line, ',') with ordinality as r(value, position);

create temporary table memory (position integer primary key, value integer not null);
create temporary table inputs (position integer primary key, value integer not null);

create function reset(phase integer, last_input integer) returns void language sql as $$
    truncate memory;
    insert into memory (position, value) select position, value from program;

    truncate inputs;
    insert into inputs (position, value) values (0, phase), (1, last_input);
$$;

create function output(t text) returns void language plpgsql as $$
    begin raise notice '%', t; end
$$;

create function debug(t text) returns void language sql as $$
    --select output(t);
$$;

create function intcode_vm(pc integer) returns integer volatile language sql as $$
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

create function opcode_1(pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns integer volatile language sql as $$
    select debug(format('writing %s to %s', read(pc + 1, mode_1) + read(pc + 2, mode_2), read(pc + 3, 1)));

    update memory
    set value = read(pc + 1, mode_1, true) + read(pc + 2, mode_2, true)
    where position = read(pc + 3, 1, true);

    select intcode_vm(pc + 4);
$$;

create function opcode_2(pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns integer volatile language sql as $$
    select debug(format('writing %s to %s', read(pc + 1, mode_1) * read(pc + 2, mode_2), read(pc + 3, 1)));

    update memory
    set value = read(pc + 1, mode_1, true) * read(pc + 2, mode_2, true)
    where position = read(pc + 3, 1, true);

    select intcode_vm(pc + 4);
$$;

create function opcode_3(pc integer, mode_1 integer) returns integer volatile language sql as $$
    select debug(format('writing 5 to %s', read(pc + 1, 1)));

    with input_value as (
        delete from inputs
        where position = (select position from inputs order by position limit 1)
        returning value
    )
    update memory
    set value = (select value from input_value)
    where position = read(pc + 1, 1, true);

    select intcode_vm(pc + 2);
$$;

create function opcode_4(pc integer, mode_1 integer) returns integer volatile language sql as $$
    select debug(format('output: %s', read(pc + 1, mode_1)));

    select read(pc + 1, mode_1, true);

    --select intcode_vm(pc + 2);
$$;

create function opcode_5(pc integer, mode_1 integer, mode_2 integer) returns integer volatile language sql as $$
    select debug(format('jump to %s if %s <> 0', read(pc + 2, mode_2), read(pc + 1, mode_1)));

    select intcode_vm(
        case read(pc + 1, mode_1, true)
            when 0 then pc + 3
            else read(pc + 2, mode_2, true)
        end);
$$;

create function opcode_6(pc integer, mode_1 integer, mode_2 integer) returns integer volatile language sql as $$
    select debug(format('jump to %s if %s = 0', read(pc + 2, mode_2), read(pc + 1, mode_1)));

    select intcode_vm(
        case read(pc + 1, mode_1, true)
            when 0 then read(pc + 2, mode_2, true)
            else pc + 3
        end);
$$;

create function opcode_7(pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns integer volatile language sql as $$
    select debug(format('writing result of %s < %s to %s', read(pc + 1, mode_1), read(pc + 2, mode_2), read(pc + 3, 1)));

    update memory
    set value = case when read(pc + 1, mode_1, true) < read(pc + 2, mode_2, true) then 1 else 0 end
    where position = read(pc + 3, 1, true);

    select intcode_vm(pc + 4);
$$;

create function opcode_8(pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns integer volatile language sql as $$
    select debug(format('writing result of %s = %s to %s', read(pc + 1, mode_1), read(pc + 2, mode_2), read(pc + 3, 1)));

    update memory
    set value = case when read(pc + 1, mode_1, true) = read(pc + 2, mode_2, true) then 1 else 0 end
    where position = read(pc + 3, 1, true);

    select intcode_vm(pc + 4);
$$;

create or replace function intcode_vm(pc integer) returns integer volatile language sql as $$
    select case opcode
        when 1 then opcode_1(pc, mode_1, mode_2, mode_3)
        when 2 then opcode_2(pc, mode_1, mode_2, mode_3)
        when 3 then opcode_3(pc, mode_1)
        when 4 then opcode_4(pc, mode_1)
        when 5 then opcode_5(pc, mode_1, mode_2)
        when 6 then opcode_6(pc, mode_1, mode_2)
        when 7 then opcode_7(pc, mode_1, mode_2, mode_3)
        when 8 then opcode_8(pc, mode_1, mode_2, mode_3)
        when 99 then null
        else null
        end
    from read_instruction(pc), debug(format('pc %s instruction %s modes %s %s %s', pc, opcode, mode_1, mode_2, mode_3));
$$;

create function try_permutations(a boolean, b boolean, c boolean, d boolean, e boolean, last_output integer) returns integer language sql as $$
    select 0/0;
$$;

create function try_permutation(phase integer, a boolean, b boolean, c boolean, d boolean, e boolean, last_output integer) returns integer language sql as $$
    select reset(phase, last_output);

    select try_permutations(a, b, c, d, e, intcode_vm(0));
$$;

create or replace function try_permutations(a boolean, b boolean, c boolean, d boolean, e boolean, last_output integer) returns integer language sql as $$
    select case when a or b or c or d or e then greatest(
        case when a then try_permutation(0, false, b, c, d, e, last_output) end,
        case when b then try_permutation(1, a, false, c, d, e, last_output) end,
        case when c then try_permutation(2, a, b, false, d, e, last_output) end,
        case when d then try_permutation(3, a, b, c, false, e, last_output) end,
        case when e then try_permutation(4, a, b, c, d, false, last_output) end
    )
    else last_output end;
$$;

select try_permutations(true, true, true, true, true, 0);

rollback;
