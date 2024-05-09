begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null, line_number integer generated always as identity);
\copy input (line) from input/11


create temporary table exploded (contents text not null, seat integer not null, x integer not null, y integer not null);
create index on exploded (x, y) where contents = '#';

create function iterate(field text, width integer) returns text volatile language sql as $$
    truncate table exploded;

    insert into exploded (contents, seat, x, y)
        select contents, seat, (seat-1) % width, (seat-1) / width
        from regexp_split_to_table(field, '') with ordinality as _(contents, seat);

    select string_agg(
        case
            when exploded.contents = 'L' and (
            select count(*)
            from exploded as neighbors
            where neighbors.x between exploded.x - 1 and exploded.x + 1
            and neighbors.y between exploded.y - 1 and exploded.y + 1
            and not (neighbors.x = exploded.x and neighbors.y = exploded.y)
            and neighbors.contents = '#') = 0 then '#'

            when exploded.contents = '#' and (
            select count(*)
            from exploded as neighbors
            where neighbors.x between exploded.x - 1 and exploded.x + 1
            and neighbors.y between exploded.y - 1 and exploded.y + 1
            and not (neighbors.x = exploded.x and neighbors.y = exploded.y)
            and neighbors.contents = '#') >= 4 then 'L'
        else contents end
    , '' order by seat)
    from exploded;
$$;


with recursive iterations as (
    select string_agg(line, '' order by line_number) as field, max(length(line)) as width, count(*) as height from input

    union all

    select iterate(field, width), width, height
    from iterations

), changed as (
    select field, field = lag(field) over () as unchanged
    from iterations
), final as (
    select field
    from changed
    where unchanged
    limit 1
), seats as (
    select regexp_split_to_table(field, '') as seat
    from final
)

select count(*) from seats where seat = '#';


rollback;
