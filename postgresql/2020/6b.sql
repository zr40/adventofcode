begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null, line_number integer not null generated always as identity);
\copy input (line) from input/6
insert into input (line) values ('');


with groups as (
    select coalesce(lag(line_number) over (order by line_number), 0) + 1 as start, line_number - 1 as end
    from input
    where line = ''
), fields as (
    select groups.end, field
    from groups
    inner join input on input.line_number between groups.start and groups.end
    cross join regexp_split_to_table(line, '') as _(field)
), fieldcounts as (
    select fields.end, field, count(*)
    from fields
    group by fields.end, field
)
select count(*)
from fieldcounts
inner join groups on fieldcounts.end = groups.end
where fieldcounts.count = groups.end - groups.start + 1;


rollback;
