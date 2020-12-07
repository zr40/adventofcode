begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null);
\copy input (line) from input/7
create temporary table contents (color text not null, contains text not null, amount integer not null);

insert into contents (color, contains, amount)
select split_part(line, ' bags contain ', 1), match[2], match[1]::integer
from input, regexp_matches(line, '(?:contain |, )(\d+) ([^,]*) bags?\.?', 'g') as match;


with recursive shiny_gold_containers as (
    select contains, amount
    from contents
    where color = 'shiny gold'

    union all

    select contents.contains, contents.amount * shiny_gold_containers.amount
    from contents
    inner join shiny_gold_containers on contents.color = shiny_gold_containers.contains
)
select sum(amount)
from shiny_gold_containers;


rollback;
