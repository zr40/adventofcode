begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text);
\copy input (line) from input/23

create temporary table state (cup integer not null, pos integer not null, move integer not null, primary key (move, pos), unique (move, cup));

insert into state (cup, pos, move)
select cup::integer, pos, 0
from input, regexp_split_to_table(line, '') with ordinality as _(cup, pos);


create function debug(input text) returns void language plpgsql as $$
    begin
        raise notice '%', input;
    end
$$;


create function move(thismove integer) returns void language sql as $$
    with pickup as (
        select cup, pos
        from state
        where move = thismove - 1
        and pos between 2 and 4
    ), destination (cup) as (
        select case
            when not exists (select from pickup where pickup.cup = (8 + state.cup - 1) % 9 + 1) then (8 + state.cup - 1) % 9 + 1
            when not exists (select from pickup where pickup.cup = (8 + state.cup - 2) % 9 + 1) then (8 + state.cup - 2) % 9 + 1
            when not exists (select from pickup where pickup.cup = (8 + state.cup - 3) % 9 + 1) then (8 + state.cup - 3) % 9 + 1
            else (8 + state.cup - 4) % 9 + 1
        end
        from state
        where move = thismove - 1 and pos = 1
    )
    insert into state (cup, pos, move)
    select cup, row_number() over (rows unbounded preceding), thismove
    from (
        (
            select cup
            from state
            where state.move = thismove - 1
            and pos > 4 and pos <= (select state.pos from state, destination where state.move = thismove - 1 and state.cup = destination.cup)
            order by state.pos
        )

        union all

        (
            select cup
            from pickup
            order by pickup.pos
        )

        union all

        (
            select cup
            from state
            where state.move = thismove - 1
            and pos > (select state.pos from state, destination where state.move = thismove - 1 and state.cup = destination.cup)
            order by state.pos
        )

        union all

        select cup
        from state
        where state.move = thismove - 1
        and pos = 1
    ) as newstate,
    debug(format(
        e'\n-- move %s --\ncups: %s\npickup: %s\ndestination: %s\n',
        thismove,
        (select string_agg(cup::text, ', ' order by pos) from state where move = thismove - 1),
        (select string_agg(cup::text, ', ' order by pos) from pickup),
        (select cup from destination)
    )) as debug;
$$;

select move(i) from generate_series(1, 100) as gs(i) \g /dev/null

select string_agg(cup::text, '' order by pos < (select pos from state where cup = 1 and move = 100), pos) from state
where cup <> 1
and move = 100;

rollback;
