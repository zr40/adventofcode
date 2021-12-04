begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity, value text not null);
\copy input (value) from input/4

create temporary table draw (number integer not null, line integer not null primary key);
create temporary table board (id integer not null, x integer not null, y integer not null, number integer not null);
create index on board(id, x);
create index on board(id, y);

insert into draw (number, line)
select number::integer, ord
from input,
     regexp_split_to_table(value, ',') with ordinality as r(number, ord)
where line = 1;

insert into board (id, x, y, number) select (line + 4) / 6, ord, (line - 2) % 6, number::integer
from input,
     regexp_split_to_table(trim(leading ' ' from value), ' +') with ordinality as r(number, ord)
where line > 1 and number <> '';

create function iterate() returns integer language plpgsql as $$
declare
    draws integer := 1;
    result integer;
    sum integer;
begin
    loop
        delete from board where id in (
            select id
            from board
            inner join draw on board.number = draw.number
            where draw.line <= draws
            group by grouping sets ((id, x), (id, y))
            having count(*) = 5
        );

        draws = draws + 1;

        if (select count(*) = 25 from board) then
            select id into result
            from board
            inner join draw on board.number = draw.number
            where draw.line <= draws
            group by grouping sets ((id, x), (id, y))
            having count(*) = 5;

            if result is not null then
                sum = sum(number)
                    from board
                    where not exists (
                        select
                        from draw
                        where draw.line <= draws
                        and draw.number = board.number
                    );

                return sum * (select number from draw where line = draws);
            end if;
        end if;
    end loop;
end
$$;

select iterate();

rollback;
