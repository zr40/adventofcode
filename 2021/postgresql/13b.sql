begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (value text not null, line integer generated always as identity);
\copy input (value) from input/13

create temporary table dots (x integer not null, y integer not null);
insert into dots (x, y)
select s[1]::integer, s[2]::integer
from input, string_to_array(value, ',') as s
where value ~ ',';

create temporary table folds (line integer, x integer, y integer);
insert into folds (line, x, y)
select
    line,
    case
    when s[1] = 'fold along x' then s[2]::integer end,
    case when s[1] = 'fold along y' then s[2]::integer end
from input, string_to_array(value, '=') as s
where value ~ '=';

create function fold (fold_x integer, fold_y integer) returns void language sql as $$
    update dots
    set
        x = case
            when fold_x is null then x
            when x < fold_x then x
            when x > fold_x then fold_x - (x - fold_x)
            end,
        y = case
            when fold_y is null then y
            when y < fold_y then y
            when y > fold_y then fold_y - (y - fold_y)
            end
    ;
$$;

select fold(x, y)
from folds;

select string_agg(case when dots is null then ' ' else '#' end, '' order by grid_x.x)
from (
    select
        max(x) as max_x,
        max(y) as max_y
    from dots
) as bounds
cross join generate_series(0, max_x) as grid_x(x)
cross join generate_series(0, max_y) as grid_y(y)
left join (select distinct x, y from dots) as dots on grid_x.x = dots.x and grid_y.y = dots.y
group by grid_y.y
order by grid_y.y;

rollback;
