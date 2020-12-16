begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text, line_number integer primary key generated always as identity);
\copy input (line) from input/16

create temporary table rules (field text not null, valid int4range not null);
insert into rules (field, valid)
select split_part(line, ':', 1), int4range(split_part(range, '-', 1)::integer, split_part(range, '-', 2)::integer, '[]')
from input, regexp_split_to_table(split_part(line, ': ', 2), ' or ') as range
where line_number < (select line_number from input where line = '' order by line_number limit 1);

create temporary table nearby_tickets (line_number integer not null, field_number integer not null, value integer not null);
insert into nearby_tickets (line_number, field_number, value)
select line_number, field_number, value::integer
from input, regexp_split_to_table(line, ',') with ordinality as field(value, field_number)
where line_number > (select line_number from input where line = 'nearby tickets:' order by line_number desc limit 1);;


select sum(value)
from nearby_tickets
where not exists (
    select
    from rules
    where valid @> value
);


rollback;
