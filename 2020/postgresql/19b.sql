begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text, line_number integer primary key generated always as identity);
\copy input (line) from input/19

create temporary table unparsed_rules (num integer primary key , contents text not null);
create temporary table parsed_rules (num integer primary key, regexp text not null);
create temporary table dependencies (num integer not null, depends integer not null, primary key (num, depends));

insert into unparsed_rules (num, contents)
select split_part(line, ': ', 1)::integer, split_part(line, ': ', 2)
from input
where line_number < (select line_number from input where line = '');

with start as (
    delete
    from unparsed_rules
    where contents ~ '"'
    returning *
)
insert into parsed_rules
select num, split_part(contents, '"', 2)
from start;

insert into dependencies
select num, dep::integer
from unparsed_rules, regexp_split_to_table(contents, '[ |]+') as r(dep)
on conflict do nothing;

delete from unparsed_rules where num = 11;
insert into unparsed_rules (num, contents)
values (11, '42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 | 42 42 42 42 42 31 31 31 31 31');

do $$
    begin
        while exists (select from unparsed_rules) loop
            with new_rules (num, regexp) as (
                select
                    unparsed_rules.num,
                    case when string_agg(r, '') ~ '\|' then '(' else '' end ||
                    string_agg(
                        case
                            when r = '|' then '|'
                            else (
                                select regexp
                                from parsed_rules
                                where parsed_rules.num = r::integer
                            )
                        end, '') ||
                    case when string_agg(r, '') ~ '\|' then ')' else '' end
                from unparsed_rules
                cross join regexp_split_to_table(unparsed_rules.contents, ' ') as r
                where not exists (
                    select
                    from dependencies
                    where unparsed_rules.num = dependencies.num
                    and not exists (
                        select
                        from parsed_rules
                        where parsed_rules.num = dependencies.depends
                    )
                )
                group by unparsed_rules.num
            ), del as (delete from unparsed_rules where exists (select from new_rules where new_rules.num = unparsed_rules.num))
            insert into parsed_rules
            select
                num,
                case num
                    when 8 then format('(%s)+', regexp)
                    when 11 then regexp
                    else regexp
                end
            from new_rules;
        end loop;
    end
$$;

--select * from dependencies;
--select * from parsed_rules where num = 11;

select count(*)
from input
inner join parsed_rules on parsed_rules.num = 0
where input.line_number > (
    select line_number
    from input
    where line = ''
)
and input.line ~ format('^%s$', parsed_rules.regexp);


rollback;
