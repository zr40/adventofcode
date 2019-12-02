begin;
create schema aoc;
set search_path to aoc;

create function fuel(mass integer) returns integer immutable language sql as $$
    select case when mass / 3 - 2 > 0 then (mass / 3 - 2) + fuel(mass / 3 - 2) else 0 end;
$$;

create temporary table input (line text);
\copy input from input/1

select sum(fuel(line::integer)) from input;

rollback;
