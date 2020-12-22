begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text, line_number integer generated always as identity);
\copy input (line) from input/22

create temporary table p1 (card integer not null, pos integer primary key generated always as identity);
create temporary table p2 (card integer not null, pos integer primary key generated always as identity);

insert into p1 (card)
select line::integer
from input
where line_number > 1 and line_number < (select line_number from input where line = '' order by line_number limit 1)
order by line_number;

insert into p2 (card)
select line::integer
from input
where line_number > (select line_number from input where line = 'Player 2:')
order by line_number;


do $$
    begin
        while (select count(*) from p1) > 0 and (select count(*) from p2) > 0 loop
            with p1draw (card) as (
                delete from p1
                where pos = (select min(pos) from p1)
                returning card
            ), p2draw (card) as (
                delete from p2
                where pos = (select min(pos) from p2)
                returning card
            ), p1winner as (
                insert into p1 (card)
                select v.card
                from p1draw, p2draw, lateral (values (p1draw.card),(p2draw.card)) as v(card)
                where p1draw.card > p2draw.card
            )
            insert into p2 (card)
            select v.card
            from p1draw, p2draw, lateral (values (p2draw.card),(p1draw.card)) as v(card)
            where p1draw.card < p2draw.card;

        end loop;

    end
$$;


select sum(card * ((select max(pos)+1 from p1) - pos)) from p1
union all
select sum(card * ((select max(pos)+1 from p2) - pos)) from p2;


rollback;
