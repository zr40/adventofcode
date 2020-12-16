begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text, line_number integer primary key generated always as identity);
\copy input (line) from input/16

create temporary table rules (field text not null, valid int4range not null, assigned_field integer);
insert into rules (field, valid)
select split_part(line, ':', 1), int4range(split_part(range, '-', 1)::integer, split_part(range, '-', 2)::integer, '[]')
from input, regexp_split_to_table(split_part(line, ': ', 2), ' or ') as range
where line_number < (select line_number from input where line = '' order by line_number limit 1);

create temporary table nearby_tickets (line_number integer not null, field_number integer not null, value integer not null);
insert into nearby_tickets (line_number, field_number, value)
select line_number, field_number, value::integer
from input, regexp_split_to_table(line, ',') with ordinality as field(value, field_number)
where line_number > (select line_number from input where line = 'nearby tickets:' order by line_number desc limit 1);;

create aggregate product(bigint) (stype=bigint, sfunc=int8mul, initcond=1);


delete from nearby_tickets where exists (
    select
    from nearby_tickets as n2
    where n2.line_number = nearby_tickets.line_number
    and not exists (select from rules where valid @> value)
);

do $$
    begin
        loop
            if (select count(*) from nearby_tickets) = 0 then
                exit;
            end if;

            with options (field, field_number, count) as (
                select rules.field, nearby_tickets.field_number, count(*) from rules
                inner join nearby_tickets on valid @> value
                where rules.assigned_field is null
                group by rules.field, nearby_tickets.field_number
            ), max_count (max) as (
                select max(count) from options
            ), assignments (field, field_number) as (
                select field, field_number
                from options, max_count
                where count = max_count.max
                and not exists (
                    select
                    from options as o2
                    where o2.field = options.field
                    and o2.field_number <> options.field_number
                    and count = max_count.max
                )
            ), u_rules as (
                update rules
                set assigned_field = assignments.field_number
                from assignments
                where rules.field = assignments.field
            )
            delete from nearby_tickets
            where exists (select from assignments where nearby_tickets.field_number = assignments.field_number);
        end loop;
    end
$$;

delete from rules where exists (
    select
    from rules as r2
    where rules.field = r2.field
    and rules.valid > r2.valid
);

select product(value::integer)
from input
cross join regexp_split_to_table(line, ',') with ordinality as field(value, field_number)
inner join rules on field.field_number = rules.assigned_field
where line_number = (select line_number+1 from input where line = 'your ticket:' order by line_number desc limit 1)
and rules.field ~ '^departure';


rollback;
