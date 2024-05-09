begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text not null, line_number integer not null generated always as identity);
\copy input (line) from input/3


select (
    select count(*)
    from input
    where substr(line, mod((line_number - 1), length(line)) + 1, 1) = '#'
) * (
    select count(*)
    from input
    where substr(line, mod((line_number - 1) * 3, length(line)) + 1, 1) = '#'
) * (
    select count(*)
    from input
    where substr(line, mod((line_number - 1) * 5, length(line)) + 1, 1) = '#'
) * (
    select count(*)
    from input
    where substr(line, mod((line_number - 1) * 7, length(line)) + 1, 1) = '#'
) * (
    select count(*)
    from input
    where substr(line, mod((line_number / 2), length(line)) + 1, 1) = '#'
    and mod(line_number, 2) = 1
);


rollback;