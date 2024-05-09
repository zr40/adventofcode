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
    or exists (select from fields where groups.end = fields.end and fields.name = 'byr' and fields.value::integer not between 1920 and 2002)
    or exists (select from fields where groups.end = fields.end and fields.name = 'iyr' and fields.value::integer not between 2010 and 2020)
    or exists (select from fields where groups.end = fields.end and fields.name = 'eyr' and fields.value::integer not between 2020 and 2030)
    or exists (select from fields where groups.end = fields.end and fields.name = 'hgt' and fields.value !~ '^\d+(in|cm)$')
    or exists (select from fields where groups.end = fields.end and fields.name = 'hgt' and fields.value ~ '^\d+cm$' and substring(fields.value from '^(\d+)cm$')::integer not between 150 and 193)
    or exists (select from fields where groups.end = fields.end and fields.name = 'hgt' and fields.value ~ '^\d+in$' and substring(fields.value from '^(\d+)in$')::integer not between 59 and 76)
    or exists (select from fields where groups.end = fields.end and fields.name = 'hcl' and fields.value !~ '^#[0-9a-f]{6}$')
    or exists (select from fields where groups.end = fields.end and fields.name = 'ecl' and fields.value not in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'))
    or exists (select from fields where groups.end = fields.end and fields.name = 'pid' and fields.value !~ '^[0-9]{9}$')
    group by groups.end
)
select (select count(*) from groups) - (select count(*) from invalid);


rollback;