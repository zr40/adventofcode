begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line integer generated always as identity, value text not null);
\copy input (value) from input/3

create temporary table work (value text not null);

create function iterate(most_common boolean) returns integer language plpgsql as $$
declare
    pos integer := 1;
    target text;
    ones integer;
    zeroes integer;
begin
    truncate work;
    insert into work select value from input;

    while (select count(*) from work) > 1 loop
        ones = (select count(*) from work where substr(value, pos, 1) = '1');
        zeroes = (select count(*) from work where substr(value, pos, 1) = '0');
        target = case when (ones >= zeroes and most_common) or (ones < zeroes and not most_common) then '1' else '0' end;

        delete from work where substr(value, pos, 1) <> target;
        pos = pos + 1;
    end loop;

    return (select ('0' || value)::bit(13)::integer from work);
end
$$;

select iterate('1') * iterate('0');

rollback;
