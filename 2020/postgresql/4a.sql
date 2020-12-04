begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null, line_number integer not null generated always as identity);
\copy input (line) from input/4
insert into input (line) values ('');


with groups as (
    select coalesce(lag(line_number) over (order by line_number), 0) + 1 as start, line_number - 1 as end
    from input
    where line = ''
), fields as (
    select groups.end, split_part(field, ':', 1) as name, split_part(field, ':', 2) as value
    from groups
    inner join input on input.line_number between groups.start and groups.end
    cross join regexp_split_to_table(line, ' ') as _(field)
), invalid as (
    select groups.end
    from groups
    cross join (values ('byr'), ('iyr'), ('eyr'), ('hgt'), ('hcl'), ('ecl'), ('pid')) as required(name)
    where not exists(select from fields where groups.end = fields.end and fields.name = required.name)
    group by groups.end
)
select (select count(*) from groups) - (select count(*) from invalid);


rollback;