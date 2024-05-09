--begin;
set client_min_messages to warning;
drop schema if exists aoc cascade;
create schema aoc;
set search_path to aoc;
set jit to off;
reset client_min_messages;

create temporary table input (line text);
\copy input (line) from input/23

create unlogged table state (cup integer primary key, clockwise_of integer null, id integer not null generated always as identity);
create index on state(clockwise_of);

insert into state (cup, clockwise_of)
select cup::integer - 1, lag(cup) over (order by pos)::integer - 1
from input, regexp_split_to_table(line, '') with ordinality as _(cup, pos);

insert into state (cup, clockwise_of)
select 9, cup
from state
where id = 9;

insert into state (cup, clockwise_of)
select n, n-1
from generate_series(10, 999999) as gs(n);

update state set clockwise_of = first.cup from state as first where state.id = 1 and first.id = (select id from state order by id desc limit 1);

analyze state;

create function debug(input text) returns void language plpgsql as $$
    begin
        raise notice '%', input;
    end
$$;

create function move(currentcup integer, cupcount integer) returns integer language sql as $$
    with recursive pickup(cup, n) as (
        select cup, 1
        from state
        where clockwise_of = currentcup

        union all

        select state.cup, pickup.n + 1
        from pickup
        inner join state on pickup.cup = state.clockwise_of
        where pickup.n < 3
    ), destination (cup) as (
        select case
            when not exists (select from pickup where pickup.cup = (cupcount + currentcup - 1) % cupcount) then (cupcount + currentcup - 1) % cupcount
            when not exists (select from pickup where pickup.cup = (cupcount + currentcup - 2) % cupcount) then (cupcount + currentcup - 2) % cupcount
            when not exists (select from pickup where pickup.cup = (cupcount + currentcup - 3) % cupcount) then (cupcount + currentcup - 3) % cupcount
            else (cupcount + currentcup - 4) % cupcount
        end
    ), remove_pickup as (
        update state
        set clockwise_of = currentcup
        from pickup
        where pickup.n = 3 and pickup.cup = state.clockwise_of
    ), update_destination as (
        update state
        set clockwise_of = pickup.cup
        from destination, pickup
        where destination.cup = state.clockwise_of
        and pickup.n = 3
    ), update_pickup as (
        update state
        set clockwise_of = destination.cup
        from pickup, destination
        where pickup.n = 1 and pickup.cup = state.cup
    )
    select
--     from debug(format(
--         e'\n-- move %s --\ncups: %s\npickup: %s\ndestination: %s\n',
--         thismove,
--         (
--             with recursive orderedstate(cup) as (
--                 select currentcup
--
--                 union all
--
--                 select state.cup
--                 from orderedstate
--                 inner join state on orderedstate.cup = state.clockwise_of
--                 where state.cup <> currentcup
--             )
--             select string_agg((cup+1)::text, ', ') from orderedstate
--         ),
--         (select string_agg((cup+1)::text, ', ') from pickup),
--         (select cup+1 from destination)
--     )) as debug
    ;

    select cup from state
    where clockwise_of = currentcup;
$$;

do $$
    declare
        thismove integer := 1;
        currentcup integer := (select cup from state where id = 1);
        cupcount integer := (select count(*)::integer from state);
    begin
        while thismove <= 10000000 loop
            if thismove % 10000 = 0 then
                commit;
                raise notice 'move % - %', thismove, clock_timestamp();
            end if;

            currentcup := move(currentcup, cupcount);
            commit;

            thismove := thismove + 1;
        end loop;

    end
$$;

select (cup1.cup::bigint+1) * (cup2.cup::bigint+1)
from state as cup1
inner join state as cup2 on cup2.clockwise_of = cup1.cup
where cup1.clockwise_of = 0;


--rollback;
drop schema aoc cascade;
