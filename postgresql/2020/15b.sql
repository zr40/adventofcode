-- Note: in a separate session, run `VACUUM aoc.numbers \watch` so that index cleanup is performed.

drop schema if exists aoc cascade;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text);
\copy input (line) from input/15

create unlogged table numbers (num integer not null primary key, turn integer not null, prevturn integer);
create unique index on numbers (turn);

insert into numbers (num, turn)
select num::integer, turn
from input, regexp_split_to_table(line, ',') with ordinality as _(num, turn);

create function turn(thisturn integer) returns void language sql as $$
    insert into numbers (num, turn, prevturn)
    select
        coalesce(numbers.turn - numbers.prevturn, 0),
        thisturn,
        null
    from numbers
    where numbers.turn = thisturn - 1
    on conflict (num) do update set prevturn = numbers.turn, turn = thisturn;
$$;

do $$
    begin
        loop
            if (select max(turn) from numbers) > 30000000 then
                exit;
            end if;

            perform turn(turn) from generate_series((select max(turn)+1 from numbers), (select max(turn)+10000 from numbers)) gs(turn);

            raise notice '% - %', (select max(turn) from numbers), clock_timestamp();
            commit;
        end loop;
    end
$$;

select num from numbers where turn = 30000000;

drop schema aoc cascade;
