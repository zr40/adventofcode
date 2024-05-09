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
    select color
    from contents
    where contains = 'shiny gold'

    union

    select contents.color
    from contents
    inner join shiny_gold_containers on contents.contains = shiny_gold_containers.color
)
select count(*)
from shiny_gold_containers;


rollback;
