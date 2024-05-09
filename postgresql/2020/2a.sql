begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null);
\copy input from input/2


with parsed as (
    select
        split_part(line, '-', 1)::integer as letter_min,
        split_part(split_part(line, '-', 2), ' ', 1)::integer as letter_max,
        split_part(split_part(line, ' ', 2), ':', 1) as letter,
        split_part(line, ': ', 2) as password
    from input
)
select count(*)
from parsed
where char_length(regexp_replace(password, '[^' || letter || ']', '', 'g')) between letter_min and letter_max;


rollback;