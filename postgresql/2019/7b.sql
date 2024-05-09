begin;
create schema aoc;
set search_path to aoc;

create temporary table input (line text);
\copy input from input/7

create temporary table program (position integer primary key, value integer not null);
insert into program (value, position) select value::integer, position - 1 from input, regexp_split_to_table(input.line, ',') with ordinality as r(value, position);

create temporary table memory (position integer, value integer not null, instance integer not null, primary key (instance, position));
create temporary table inputs (position integer, value integer not null, instance integer not null, primary key (instance, position));
create temporary table pcs (pc integer not null, instance integer primary key);
create temporary table active_instance (instance integer not null);

create function reset(phase_0 integer, phase_1 integer, phase_2 integer, phase_3 integer, phase_4 integer) returns void language sql as $$
    truncate memory;
    insert into memory (position, value, instance) select position, value, generate_series(0,4) from program;

    truncate inputs;
    insert into inputs (position, value, instance) values (0, phase_0, 0), (0, phase_1, 1), (0, phase_2, 2), (0, phase_3, 3), (0, phase_4, 4), (1, 0, 0);

    truncate pcs;
    insert into pcs (pc, instance) select 0, generate_series(0,4);

    truncate active_instance;
    insert into active_instance (instance) select 0;
$$;

create function output(t text) returns void language plpgsql as $$
    begin raise notice '%', t; end
$$;

create function debug(t text) returns void language sql as $$
    -- select output(t);
$$;

create function intcode_vm() returns integer volatile language sql as $$
    select 0/0;
$$;

create function read_instruction(active_instance out integer, pc out integer, out opcode integer, out mode_1 integer, out mode_2 integer, out mode_3 integer) stable language sql as $$
    select debug(format(
        'instance %s: reading instruction at pc %s: %s',
        ai.instance,
        pc,
        value
    ))
    from memory
    inner join pcs on memory.instance = pcs.instance and memory.position = pcs.pc
    inner join active_instance ai on pcs.instance = ai.instance;

    select ai.instance, pcs.pc, value % 100, value / 100 % 10, value / 1000 % 10, value / 10000
    from memory
    inner join pcs on memory.instance = pcs.instance and memory.position = pcs.pc
    inner join active_instance ai on pcs.instance = ai.instance;
$$;

create function jump(active_instance integer, new_pc integer) returns void language sql as $$
    update pcs set pc = new_pc where instance = active_instance;
$$;

create function read(active_instance integer, read_position integer, mode integer, silent boolean = false) returns integer stable language sql as $$
    select debug(format(
        'instance %s: reading %s in mode %s: %s',
        active_instance,
        read_position,
        mode,
    value))
    from memory
    where instance = active_instance and position = read_position and not silent;

    select case mode
        when 1 then value
        when 0 then read(active_instance, value, 1, silent)
    end
    from memory
    where instance = active_instance and position = read_position;
$$;

create function opcode_1(active_instance integer, pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns integer volatile language sql as $$
    select debug(format(
        'instance %s: writing %s to %s',
        active_instance,
        read(active_instance, pc + 1, mode_1) + read(active_instance, pc + 2, mode_2),
        read(active_instance, pc + 3, 1)
    ));

    update memory
    set value = read(active_instance, pc + 1, mode_1, true) + read(active_instance, pc + 2, mode_2, true)
    where instance = active_instance and position = read(active_instance, pc + 3, 1, true);

    select jump(active_instance, pc + 4);
    select intcode_vm();
$$;

create function opcode_2(active_instance integer, pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns integer volatile language sql as $$
    select debug(format(
        'instance %s: writing %s to %s',
        active_instance,
        read(active_instance, pc + 1, mode_1) * read(active_instance, pc + 2, mode_2),
        read(active_instance, pc + 3, 1)
    ));

    update memory
    set value = read(active_instance, pc + 1, mode_1, true) * read(active_instance, pc + 2, mode_2, true)
    where position = read(active_instance, pc + 3, 1, true);

    select jump(active_instance, pc + 4);
    select intcode_vm();
$$;

create function opcode_3(active_instance integer, pc integer, mode_1 integer) returns integer volatile language sql as $$
    with input_value as (
        select value from inputs
        where instance = active_instance and position = (select position from inputs where instance = active_instance order by position limit 1)
    )
    select debug(format(
        'instance %s: writing input value %s to %s',
        active_instance,
        (select value from input_value),
        read(active_instance, pc + 1, 1)
    ));

    with input_value as (
        delete from inputs
        where instance = active_instance and position = (select position from inputs where instance = active_instance order by position limit 1)
        returning value
    )
    update memory
    set value = (select value from input_value)
    where instance = active_instance and position = read(active_instance, pc + 1, 1, true);

    select jump(active_instance, pc + 2);
    select intcode_vm();
$$;

create function opcode_4(active_instance integer, pc integer, mode_1 integer) returns integer volatile language sql as $$
    select debug(format(
        'instance %s: output: %s',
        active_instance,
        read(active_instance, pc + 1, mode_1)
    ));

    insert into inputs (position, value, instance)
    select 10, read(active_instance, pc + 1, mode_1, true), case when active_instance = 4 then 0 else active_instance + 1 end;

    select jump(active_instance, pc + 2);
    update active_instance set instance = case when instance = 4 then 0 else instance + 1 end;
    select intcode_vm();
$$;

create function opcode_5(active_instance integer, pc integer, mode_1 integer, mode_2 integer) returns integer volatile language sql as $$
    select debug(format(
        'instance %s: jump to %s if %s <> 0',
        active_instance,
        read(active_instance, pc + 2, mode_2),
        read(active_instance, pc + 1, mode_1)
    ));

    select jump(
        active_instance,
        case read(active_instance, pc + 1, mode_1, true)
            when 0 then pc + 3
            else read(active_instance, pc + 2, mode_2, true)
        end);
    select intcode_vm();
$$;

create function opcode_6(active_instance integer, pc integer, mode_1 integer, mode_2 integer) returns integer volatile language sql as $$
    select debug(format(
        'instance %s: jump to %s if %s = 0',
        active_instance,
        read(active_instance, pc + 2, mode_2),
        read(active_instance, pc + 1, mode_1)
    ));

    select jump(
        active_instance,
        case read(active_instance, pc + 1, mode_1, true)
            when 0 then read(active_instance, pc + 2, mode_2, true)
            else pc + 3
        end);
    select intcode_vm();
$$;

create function opcode_7(active_instance integer, pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns integer volatile language sql as $$
    select debug(format(
        'instance %s: writing result of %s < %s to %s',
        active_instance,
        read(active_instance, pc + 1, mode_1),
        read(active_instance, pc + 2, mode_2),
        read(active_instance, pc + 3, 1)
    ));

    update memory
    set value = case when read(active_instance, pc + 1, mode_1, true) < read(active_instance, pc + 2, mode_2, true) then 1 else 0 end
    where instance = active_instance and position = read(active_instance, pc + 3, 1, true);

    select jump(active_instance, pc + 4);
    select intcode_vm();
$$;

create function opcode_8(active_instance integer, pc integer, mode_1 integer, mode_2 integer, mode_3 integer) returns integer volatile language sql as $$
    select debug(format(
        'instance %s: writing result of %s = %s to %s',
        active_instance,
        read(active_instance, pc + 1, mode_1),
        read(active_instance, pc + 2, mode_2),
        read(active_instance, pc + 3, 1)
    ));

    update memory
    set value = case when read(active_instance, pc + 1, mode_1, true) = read(active_instance, pc + 2, mode_2, true) then 1 else 0 end
    where position = read(active_instance, pc + 3, 1, true);

    select jump(active_instance, pc + 4);
    select intcode_vm();
$$;

create or replace function intcode_vm() returns integer volatile language sql as $$
    select case opcode
        when 1 then opcode_1(active_instance, pc, mode_1, mode_2, mode_3)
        when 2 then opcode_2(active_instance, pc, mode_1, mode_2, mode_3)
        when 3 then opcode_3(active_instance, pc, mode_1)
        when 4 then opcode_4(active_instance, pc, mode_1)
        when 5 then opcode_5(active_instance, pc, mode_1, mode_2)
        when 6 then opcode_6(active_instance, pc, mode_1, mode_2)
        when 7 then opcode_7(active_instance, pc, mode_1, mode_2, mode_3)
        when 8 then opcode_8(active_instance, pc, mode_1, mode_2, mode_3)
        when 99 then null
        else null
        end
    from read_instruction(), debug(format('instance %s pc %s instruction %s modes %s %s %s', active_instance, pc, opcode, mode_1, mode_2, mode_3));
$$;

create function try_permutation(phase_0 integer, phase_1 integer, phase_2 integer, phase_3 integer, phase_4 integer) returns integer language sql as $$
    select output(format('trying permutation %s %s %s %s %s', phase_0, phase_1, phase_2, phase_3, phase_4));

    select reset(phase_0, phase_1, phase_2, phase_3, phase_4);

    select intcode_vm();

    select output(format('permutation result: %s', inputs.value))
    from inputs;

    select value from inputs;
$$;

create function try_permutations() returns integer language sql as $$
    select max(try_permutation(a,b,c,d,e))
    from
         generate_series(5,9) as a,
         generate_series(5,9) as b,
         generate_series(5,9) as c,
         generate_series(5,9) as d,
         generate_series(5,9) as e
    where a <> b and a <> c and a <> d and a <> e
    and b <> c and b <> d and b <> e
    and c <> d and c <> e
    and d <> e;
$$;

select try_permutations();

rollback;
