begin;
create schema aoc2;
set search_path to aoc2;
set jit to off;

create temporary table input (line text, line_number integer generated always as identity);
\copy input (line) from input/22

create temporary table deck (p1 integer[] not null, p2 integer[] not null, game integer primary key);
create temporary table decks_seen (p1 integer[] not null, p2 integer[] not null, game integer not null, primary key (game, p1, p2));
create temporary table loop_seen (p1 integer[] not null, p2 integer[] not null, primary key (p1, p2));
create sequence game_seq;

with p1(deck) as (
    select array_agg(line::integer order by line_number)
    from input
    where line_number > 1
    and line_number < (select line_number from input where line = '' order by line_number limit 1)
), p2(deck) as (
    select array_agg(line::integer order by line_number)
    from input
    where line_number > (select line_number from input where line = 'Player 2:' order by line_number limit 1)
)
insert into deck (p1, p2, game)
select p1.deck, p2.deck, 0
from p1, p2;


create function debug(msg text) returns void language plpgsql as $$
    begin
        raise notice '%', msg;
    end
$$;


create function round(initstate integer, initp1deck integer[], initp2deck integer[]) returns boolean language plpgsql as $$
    declare
        thisgame integer := nextval('game_seq');
        round integer := 0;
    begin
        insert into deck (p1, p2, game)
        select initp1deck, initp2deck, thisgame;

        raise notice e'starting game % from % - % %', thisgame, initstate, (select p1 from deck where game = thisgame), (select p2 from deck where game = thisgame);

        if (select (select max(c) from unnest(p1) as u(c)) > (select max(c) from unnest(p2) as u(c)) from deck where game = thisgame and thisgame > 1) then
            raise notice e'largest card, p1 wins game %. back to %', thisgame, initstate;
            return true;
        end if;

        while (select p1 <> '{}' and p2 <> '{}' from deck where game = thisgame) loop
            if exists (select from decks_seen as ds, deck where ds.p1 = deck.p1 and ds.p2 = deck.p2 and ds.game = thisgame and deck.game = thisgame) then
                raise notice 'infinite loop, p1 wins game %. back to %. decks: % %', thisgame, initstate, (select p1 from deck where game = thisgame), (select p2 from deck where game = thisgame);
                return true;
            end if;

            insert into decks_seen(p1, p2, game)
            select p1, p2, thisgame from deck where game = thisgame;

            round := round + 1;

            with draw (p1card, p2card, p1deck, p2deck) as (
                select p1[1], p2[1], p1[2:], p2[2:] from deck where game = thisgame
            ), winner (p1) as (
                select case
                    when p1card <= cardinality(p1deck) and p2card <= cardinality(p2deck)
                    then round(thisgame, p1deck[:p1card], p2deck[:p2card])
                    else p1card > p2card
                end
                from draw
            )
            update deck
            set p1 = case when winner.p1 then p1deck || p1card || p2card else p1deck end,
                p2 = case when not winner.p1 then p2deck || p2card || p1card else p2deck end
            from draw, winner
            where game = thisgame;

        end loop;

        raise notice 'empty deck, p% wins game %, back to %', case when (select p2 = '{}' from deck where game = thisgame) then 1 else 2 end, thisgame, initstate;

        return (select p2 = '{}' from deck where game = thisgame);
    end
$$;

select round(0, p1, p2) from deck where game = 0;


with v(val) as (
    select card * row_number() over (order by pos desc)
    from deck, unnest(p1 || p2) with ordinality as u(card, pos)
    where game = 1
)
select sum(val) from v;

select * from deck where game = 1;

rollback;
