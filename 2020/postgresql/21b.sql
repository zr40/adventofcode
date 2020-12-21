begin;
create schema aoc;
set search_path to aoc;
set jit to off;

create temporary table input (line text, line_number integer generated always as identity);
\copy input (line) from input/21

create temporary table ingredients (line_number integer not null, ingredient text not null);
create index on ingredients(ingredient);
insert into ingredients
select line_number, regexp_split_to_table(split_part(line, ' (contains ', 1), ' ')
from input;

create temporary table allergens (line_number integer not null, allergen text not null);
create index on allergens(allergen);
insert into allergens
select line_number, regexp_split_to_table(split_part(split_part(line, ' (contains ', 2), ')', 1), ', ')
from input;


create temporary table unique_ingredients (ingredient text primary key);
create temporary table unique_allergens (allergen text primary key);
insert into unique_ingredients select distinct ingredient from ingredients;
insert into unique_allergens select distinct allergen from allergens;


create function pass(_ integer) returns table (allergen text, ingredient text) language sql as $$
    with result (allergen, ingredient) as (
        select unique_allergens.allergen, (array_agg(unique_ingredients.ingredient))[1], count(*)
        from unique_allergens
                 cross join unique_ingredients
        where not exists(
                select
                from allergens
                where allergens.allergen = unique_allergens.allergen
                  and not exists(
                        select
                        from ingredients
                        where ingredients.ingredient = unique_ingredients.ingredient
                          and ingredients.line_number = allergens.line_number
                    )
            )
        group by allergen
        having count(*) = 1
    ), del_ua as (
        delete from unique_allergens using result where unique_allergens.allergen = result.allergen
    ), del_a as (
        delete from allergens using result where allergens.allergen = result.allergen
    ), del_ui as (
        delete from unique_ingredients using result where unique_ingredients.ingredient = result.ingredient
    ), del_i as (
        delete from ingredients using result where ingredients.ingredient = result.ingredient
    )
    select allergen, ingredient from result;
$$;

select string_agg(ingredient, ',' order by allergen)
from generate_series(0, 10) as _(i), pass(i) as p;


rollback;
