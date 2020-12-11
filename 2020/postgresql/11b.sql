begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null, line_number integer generated always as identity);
\copy input (line) from input/11


create temporary table exploded (contents text not null, seat integer not null, x integer not null, y integer not null);
create index on exploded (x, y) where contents = '#';

with params as (select string_agg(line, '' order by line_number) as field, max(length(line)) as width, count(*) as height from input)

insert into exploded
select contents, seat, (seat-1) % width, (seat-1) / width
from params, regexp_split_to_table(field, '') with ordinality as _(contents, seat);

create temporary table neighbors (x integer not null, y integer not null, target_x integer not null, target_y integer not null);
create index on neighbors (x, y, target_x, target_y);

with recursive neighbor_candidate (x, y, direction_x, direction_y, target_x, target_y, found) as (
    select source.x, source.y, direction.x, direction.y, target.x, target.y, target.contents = 'L'
    from exploded as source
    cross join (values
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1)
    ) as direction (x, y)
    inner join exploded as target on source.x + direction.x = target.x and source.y + direction.y = target.y

    union all

    select source.x, source.y, direction_x, direction_y, target.x, target.y, target.contents = 'L'
    from neighbor_candidate as source
    inner join exploded as target on source.target_x + source.direction_x = target.x and source.target_y + source.direction_y = target.y
    where not found
)
insert into neighbors (x, y, target_x, target_y)
select x, y, target_x, target_y from neighbor_candidate
where found;

create function iterate(field text, width integer) returns text volatile language sql as $$
    truncate table exploded;

    insert into exploded (contents, seat, x, y)
        select contents, seat, (seat-1) % width, (seat-1) / width
        from regexp_split_to_table(field, '') with ordinality as _(contents, seat);

    select string_agg(
        case
            when exploded.contents = 'L' and (
            select count(*)
            from neighbors
            inner join exploded as neighbor_contents on neighbors.target_x = neighbor_contents.x and neighbors.target_y = neighbor_contents.y
            where neighbors.x = exploded.x and neighbors.y = exploded.y
            and neighbor_contents.contents = '#') = 0 then '#'

            when exploded.contents = '#' and (
            select count(*)
            from neighbors
            inner join exploded as neighbor_contents on neighbors.target_x = neighbor_contents.x and neighbors.target_y = neighbor_contents.y
            where neighbors.x = exploded.x and neighbors.y = exploded.y
            and neighbor_contents.contents = '#') >= 5 then 'L'
        else contents end
    , '' order by seat)
    from exploded;
$$;


with recursive iterations as (
    select string_agg(line, '  ' order by line_number) as field, max(length(line) + 2) as width, count(*) as height from input

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
