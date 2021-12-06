begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity, value text not null);
\copy input (value) from input/6

with recursive initial(timer, count) as (
    select age::integer, count(*)
    from input, regexp_split_to_table(value, ',') as r(age)
    group by age::integer
), fish(day, count_0, count_1, count_2, count_3, count_4, count_5, count_6, count_7, count_8) as (
    select
        0,
        coalesce((select count from initial where timer = 0), 0),
        coalesce((select count from initial where timer = 1), 0),
        coalesce((select count from initial where timer = 2), 0),
        coalesce((select count from initial where timer = 3), 0),
        coalesce((select count from initial where timer = 4), 0),
        coalesce((select count from initial where timer = 5), 0),
        coalesce((select count from initial where timer = 6), 0),
        coalesce((select count from initial where timer = 7), 0),
        coalesce((select count from initial where timer = 8), 0)
    union all
    select
        day + 1,
        count_1,
        count_2,
        count_3,
        count_4,
        count_5,
        count_6,
        count_7 + count_0,
        count_8,
        count_0
    from fish
    where day < 256
)
select
    day,
    count_0 + count_1 + count_2 + count_3 + count_4 + count_5 + count_6 + count_7 + count_8
    from fish
    where day in (80, 256);

rollback;
