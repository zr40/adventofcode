begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null, line_number integer not null generated always as identity);
\copy input (line) from input/3


select count(*)
from input
where substr(line, mod((line_number - 1) * 3, length(line)) + 1, 1) = '#';


rollback;