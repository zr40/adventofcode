begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity primary key, value text not null);
\copy input (value) from input/8

create temporary table digits (line integer, signals text not null, digit integer, unique (line, digit));
create temporary table outputs (line integer, signals text not null, ord integer not null);

insert into digits (line, signals)
select line, string_agg(signal, '' order by signal)
from input, string_to_table(value, ' ') with ordinality as p(signals, ord), regexp_split_to_table(signals, '') as r(signal)
where ord <= 10
group by line, ord
;

insert into outputs (line, signals, ord)
select line, string_agg(signal, '' order by signal), ord
from input, string_to_table(value, ' ') with ordinality as p(signals, ord), regexp_split_to_table(signals, '') as r(signal)
where ord > 11
group by line, ord
;

update digits set digit = 1 where length(signals) = 2;
update digits set digit = 7 where length(signals) = 3;
update digits set digit = 4 where length(signals) = 4;
update digits set digit = 8 where length(signals) = 7;

update digits as candidate set digit = 9
where length(candidate.signals) = 6
and (
    select bool_and(candidate.signals ~ signal)
    from digits as four, regexp_split_to_table(four.signals, '') as r(signal)
    where four.line = candidate.line and four.digit = 4
)
;

update digits as candidate set digit = 6
where length(candidate.signals) = 6
and (
    select count(*) = 1
    from digits as one, regexp_split_to_table(one.signals, '') as r(signal)
    where one.line = candidate.line and one.digit = 1 and candidate.signals ~ signal
);

update digits set digit = 0 where length(signals) = 6 and digit is null;

update digits as candidate set digit = 3
where length(candidate.signals) = 5
and (
    select count(*) = 2
    from digits as one, regexp_split_to_table(one.signals, '') as r(signal)
    where one.line = candidate.line and one.digit = 1 and candidate.signals ~ signal
);

update digits as candidate set digit = 5
where length(candidate.signals) = 5
and (
    select count(*) = 5
    from digits as six, regexp_split_to_table(six.signals, '') as r(signal)
    where six.line = candidate.line and six.digit = 6 and candidate.signals ~ signal
) and (
    select count(*) = 1
    from digits as one, regexp_split_to_table(one.signals, '') as r(signal)
    where one.line = candidate.line and one.digit = 1 and candidate.signals ~ signal
);

update digits set digit = 2 where digit is null;

select sum(sum) from (
    select outputs.line, sum(digits.digit * case outputs.ord when 12 then 1000 when 13 then 100 when 14 then 10 else 1 end)
    from outputs
    inner join digits on outputs.signals = digits.signals and outputs.line = digits.line
    group by outputs.line
) as _
;

rollback;
