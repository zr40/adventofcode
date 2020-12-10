begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (jolts integer primary key);
\copy input (jolts) from input/10


with recursive arrangements (thisjolt1, pathstohere1, thisjolt2, pathstohere2, thisjolt3, pathstohere3) as (
    select 0, 1::bigint, 0, 0::bigint, 0, 0::bigint

    union all

    select
        nextjolt.jolts,
        case when nextjolt.jolts - arrangements.thisjolt1 <= 3 then arrangements.pathstohere1 else 0 end +
        case when nextjolt.jolts - arrangements.thisjolt2 <= 3 then arrangements.pathstohere2 else 0 end +
        case when nextjolt.jolts - arrangements.thisjolt3 <= 3 then arrangements.pathstohere3 else 0 end,

        thisjolt1, pathstohere1, thisjolt2, pathstohere2
    from arrangements
    cross join lateral (
        select jolts from input
        where jolts > thisjolt1
        order by jolts
        limit 1
    ) as nextjolt
)
select pathstohere1 from arrangements;


rollback;
