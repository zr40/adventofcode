begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (jolts bigint primary key);
\copy input (jolts) from input/10
insert into input (jolts) values (0);


create aggregate product(bigint) (stype=bigint, sfunc=int8mul, initcond=1);

with differences (diff) as (
    select coalesce(lead(jolts) over (order by jolts) - jolts, 3)
    from input
), diffcount (count) as (
    select count(*)
    from differences
    group by diff
)
select product(count) from diffcount;


rollback;
