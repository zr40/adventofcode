begin;
create schema aoc_21b;
set search_path to aoc_21b;

create table input (line text);
\copy input from 'input/21'

create table rule_2 (
  in11 boolean not null,
  in12 boolean not null,
  in21 boolean not null,
  in22 boolean not null,
  out11 boolean not null,
  out12 boolean not null,
  out13 boolean not null,
  out21 boolean not null,
  out22 boolean not null,
  out23 boolean not null,
  out31 boolean not null,
  out32 boolean not null,
  out33 boolean not null,
  primary key (in11, in12, in21, in22)
);

insert into rule_2
select
  line[1] = '#',
  line[2] = '#',
  line[4] = '#',
  line[5] = '#',

  line[10] = '#',
  line[11] = '#',
  line[12] = '#',
  line[14] = '#',
  line[15] = '#',
  line[16] = '#',
  line[18] = '#',
  line[19] = '#',
  line[20] = '#'
from (select regexp_split_to_array(line, '') as line from input where length(line) = 20) as _;

insert into rule_2
select in21, in22, in11, in12, out11, out12, out13, out21, out22, out23, out31, out32, out33
from rule_2
on conflict do nothing;

insert into rule_2
select in12, in11, in22, in21, out11, out12, out13, out21, out22, out23, out31, out32, out33
from rule_2
on conflict do nothing;

insert into rule_2
select in12, in22, in11, in21, out11, out12, out13, out21, out22, out23, out31, out32, out33
from rule_2
on conflict do nothing;

create table rule_3 (
  in11 boolean not null,
  in12 boolean not null,
  in13 boolean not null,
  in21 boolean not null,
  in22 boolean not null,
  in23 boolean not null,
  in31 boolean not null,
  in32 boolean not null,
  in33 boolean not null,
  out11 boolean not null,
  out12 boolean not null,
  out13 boolean not null,
  out14 boolean not null,
  out21 boolean not null,
  out22 boolean not null,
  out23 boolean not null,
  out24 boolean not null,
  out31 boolean not null,
  out32 boolean not null,
  out33 boolean not null,
  out34 boolean not null,
  out41 boolean not null,
  out42 boolean not null,
  out43 boolean not null,
  out44 boolean not null,
  primary key (in11, in12, in13, in21, in22, in23, in31, in32, in33)
);

insert into rule_3
select
  line[1] = '#',
  line[2] = '#',
  line[3] = '#',
  line[5] = '#',
  line[6] = '#',
  line[7] = '#',
  line[9] = '#',
  line[10] = '#',
  line[11] = '#',

  line[16] = '#',
  line[17] = '#',
  line[18] = '#',
  line[19] = '#',
  line[21] = '#',
  line[22] = '#',
  line[23] = '#',
  line[24] = '#',
  line[26] = '#',
  line[27] = '#',
  line[28] = '#',
  line[29] = '#',
  line[31] = '#',
  line[32] = '#',
  line[33] = '#',
  line[34] = '#'
from (select regexp_split_to_array(line, '') as line from input where length(line) = 34) as _;

insert into rule_3
select in31, in32, in33, in21, in22, in23, in11, in12, in13, out11, out12, out13, out14, out21, out22, out23, out24, out31, out32, out33, out34, out41, out42, out43, out44
from rule_3
on conflict do nothing;

insert into rule_3
select in13, in12, in11, in23, in22, in21, in33, in32, in31, out11, out12, out13, out14, out21, out22, out23, out24, out31, out32, out33, out34, out41, out42, out43, out44
from rule_3
on conflict do nothing;

insert into rule_3
select in13, in23, in33, in12, in22, in32, in11, in21, in31, out11, out12, out13, out14, out21, out22, out23, out24, out31, out32, out33, out34, out41, out42, out43, out44
from rule_3
on conflict do nothing;

create table rule_3_split (
  in11 boolean not null,
  in12 boolean not null,
  in13 boolean not null,
  in21 boolean not null,
  in22 boolean not null,
  in23 boolean not null,
  in31 boolean not null,
  in32 boolean not null,
  in33 boolean not null,
  out11 boolean not null,
  out12 boolean not null,
  out21 boolean not null,
  out22 boolean not null,
  x int not null,
  y int not null,
  primary key (in11, in12, in13, in21, in22, in23, in31, in32, in33, x, y)
);

insert into rule_3_split
select in11, in12, in13, in21, in22, in23, in31, in32, in33, out11, out12, out21, out22, 0, 0
from rule_3;
insert into rule_3_split
select in11, in12, in13, in21, in22, in23, in31, in32, in33, out13, out14, out23, out24, 0, 1
from rule_3;
insert into rule_3_split
select in11, in12, in13, in21, in22, in23, in31, in32, in33, out31, out32, out41, out42, 1, 0
from rule_3;
insert into rule_3_split
select in11, in12, in13, in21, in22, in23, in31, in32, in33, out33, out34, out43, out44, 1, 1
from rule_3;

create table state_2 (
  c11 boolean not null,
  c12 boolean not null,
  c21 boolean not null,
  c22 boolean not null,
  cx int not null,
  cy int not null,
  primary key (cx, cy)
);

create table state_3 (
  c11 boolean not null,
  c12 boolean not null,
  c13 boolean not null,
  c21 boolean not null,
  c22 boolean not null,
  c23 boolean not null,
  c31 boolean not null,
  c32 boolean not null,
  c33 boolean not null,
  cx int not null,
  cy int not null,
  primary key (cx, cy)
);

create function iterate_2() returns void language sql as $$
  truncate state_3;

  insert into state_3
  select
    out11, out12, out13,
    out21, out22, out23,
    out31, out32, out33,
    cx, cy
  from state_2
  inner join rule_2 on (c11, c12, c21, c22) = (in11, in12, in21, in22);
$$;

create function iterate_3() returns void language sql as $$
  truncate state_2;

  insert into state_2
  select
    out11, out12,
    out21, out22,
    cx * 2 + x, cy * 2 + y
  from state_3
  inner join rule_3_split on (c11, c12, c13, c21, c22, c23, c31, c32, c33) = (in11, in12, in13, in21, in22, in23, in31, in32, in33);
$$;

create function transfer_3_to_2() returns void language sql as $$
  truncate state_2;

  insert into state_2
  select
    unnest(array[s11.c11, s11.c13, s12.c12, s11.c31, s11.c33, s12.c32, s21.c21, s21.c23, s22.c22]),
    unnest(array[s11.c12, s12.c11, s12.c13, s11.c32, s12.c31, s12.c33, s21.c22, s22.c21, s22.c23]),

    unnest(array[s11.c21, s11.c23, s12.c22, s21.c11, s21.c13, s22.c12, s21.c31, s21.c33, s22.c32]),
    unnest(array[s11.c22, s12.c21, s12.c23, s21.c12, s22.c11, s22.c13, s21.c32, s22.c31, s22.c33]),

    s11.cx * 3 + unnest(array[0,0,0,1,1,1,2,2,2]),
    s11.cy * 3 + unnest(array[0,1,2,0,1,2,0,1,2])

  from state_3 as s11
  inner join state_3 as s12 on (s11.cx, s11.cy) = (s12.cx, s12.cy - 1)
  inner join state_3 as s21 on (s11.cx, s11.cy) = (s21.cx - 1, s21.cy)
  inner join state_3 as s22 on (s11.cx, s11.cy) = (s22.cx - 1, s22.cy - 1)

  where s11.cx % 2 = 0 and s11.cy % 2 = 0;
$$;

insert into state_3 values (
  false, true, false,
  false, false, true,
  true, true, true,
  0, 0
);

-- select iterate_3(), iterate_2(), transfer_3_to_2(), iterate_2() from generate_series(0,6);
-- (does not finish within reasonable time)

select
  iterate_3(), iterate_2(), transfer_3_to_2(), iterate_2(),
  iterate_3(), iterate_2(), transfer_3_to_2(), iterate_2(),
  iterate_3(), iterate_2(), transfer_3_to_2(), iterate_2(),
  iterate_3(), iterate_2(), transfer_3_to_2(), iterate_2(),
  iterate_3(), iterate_2(), transfer_3_to_2(), iterate_2(),
  iterate_3(), iterate_2(), transfer_3_to_2(), iterate_2();

select sum(
  c11::integer + c12::integer + c13::integer +
  c21::integer + c22::integer + c23::integer +
  c31::integer + c32::integer + c33::integer
) from state_3;

rollback;
