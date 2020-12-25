begin;
create schema aoc;
set search_path to aoc;
set jit to off;
reset client_min_messages;

create temporary table input (line text not null);
\copy input (line) from input/25


with recursive loops (pubkey, subject, loopsize) as (
    select line::integer, 1, 0
    from input

    union all

    select pubkey, (subject * 7) % 20201227, loopsize+1
    from loops

    where pubkey <> subject
), encryption_key (pubkey, subject, loops_remaining) as (
    select door.pubkey, 1::bigint, card.loopsize
    from loops as door, loops as card
    where door.pubkey <> card.pubkey
    and door.pubkey = door.subject
    and card.pubkey = card.subject
    and card.loopsize <= door.loopsize

    union all

    select pubkey, (subject * pubkey) % 20201227, loops_remaining-1
    from encryption_key
    where loops_remaining >= 0
)
select subject from encryption_key
where loops_remaining = 0;


rollback;
