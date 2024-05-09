begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity primary key, value text not null);
\copy input (value) from input/8

create temporary table digits (line integer, signals text not null, digit integer, primary key (line, signals));
create temporary table outputs (line integer, signals text not null);

insert into digits (line, signals)
select line, string_agg(signal, '' order by signal)
from input, string_to_table(value, ' ') with ordinality as p(signals, ord), regexp_split_to_table(signals, '') as r(signal)
where ord <= 10
group by line, ord
;

insert into outputs (line, signals)
select line, string_agg(signal, '' order by signal)
from input, string_to_table(value, ' ') with ordinality as p(signals, ord), regexp_split_to_table(signals, '') as r(signal)
where ord > 11
group by line, ord
;

update digits set digit = 1 where length(signals) = 2;
update digits set digit = 7 where length(signals) = 3;
update digits set digit = 4 where length(signals) = 4;
update digits set digit = 8 where length(signals) = 7;

select count(*)
from outputs
inner join digits on outputs.line = digits.line and outputs.signals = digits.signals
where digit in (1, 4, 7, 8);

rollback;
