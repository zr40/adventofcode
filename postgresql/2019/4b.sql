begin;

select count(*) from (
    select
        num,
        num / 100000 as d1,
        num % 100000 / 10000 as d2,
        num % 10000 / 1000 as d3,
        num % 1000 / 100 as d4,
        num % 100 / 10 as d5,
        num % 10 as d6
    from generate_series(246515, 739105) num
) as _
where (d1 <= d2 and d2 <= d3 and d3 <= d4 and d4 <= d5 and d5 <= d6)
and (
    (d1 = d2 and d2 <> d3) or
    (d1 <> d2 and d2 = d3 and d3 <> d4) or
    (d2 <> d3 and d3 = d4 and d4 <> d5) or
    (d3 <> d4 and d4 = d5 and d5 <> d6) or
    (d4 <> d5 and d5 = d6)
);

rollback;
