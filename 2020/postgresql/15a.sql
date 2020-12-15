begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text);
\copy input (line) from input/15

create temporary table numbers (num integer not null, turn integer primary key generated always as identity);
create index on numbers (num, turn);

insert into numbers (num)
select num::integer
from input, regexp_split_to_table(line, ',') as _(num);

create function turn() returns void language sql as $$
    with last_num (num, turn) as (
        select num, turn from numbers
        order by turn desc
        limit 1
    ), num_prev (turn) as (
        select numbers.turn
        from numbers, last_num
        where numbers.num = last_num.num and numbers.turn < last_num.turn
        order by numbers.turn desc
        limit 1
    )
    insert into numbers (num)
    select case when num_prev.turn is null then 0 else last_num.turn - num_prev.turn end
    from last_num
    left join num_prev on true;
$$;

select turn() from generate_series(4, 2020) \g /dev/null

select num from numbers where turn = 2020;


rollback;
