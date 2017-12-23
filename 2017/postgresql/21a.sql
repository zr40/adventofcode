begin;
create schema aoc_20a;
set search_path to aoc_20a;

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

with i0 (c11, c12, c13, c21, c22, c23, c31, c32, c33, cx, cy) as (
  select
    false, true, false,
    false, false, true,
    true, true, true,
    0, 0
),
-- block size 3x3, block count 1x1, cell count 3x3
i1 (c11, c12, c21, c22, cx, cy) as (
  select
    out11, out12,
    out21, out22,
    cx * 2 + x, cy * 2 + y
  from i0
  inner join rule_3_split on (c11, c12, c13, c21, c22, c23, c31, c32, c33) = (in11, in12, in13, in21, in22, in23, in31, in32, in33)
),
-- block size 2x2, block count 2x2, cell count 4x4
i2 (c11, c12, c13, c21, c22, c23, c31, c32, c33, cx, cy) as (
  select
    out11, out12, out13,
    out21, out22, out23,
    out31, out32, out33,
    cx, cy
  from i1
  inner join rule_2 on (c11, c12, c21, c22) = (in11, in12, in21, in22)
),
-- block size 3x3, block count 2x2, cell count 6x6
i2_split (c11, c12, c21, c22, cx, cy) as (
  select
    unnest(array[i2_11.c11, i2_11.c13, i2_12.c12, i2_11.c31, i2_11.c33, i2_12.c32, i2_21.c21, i2_21.c23, i2_22.c22]),
    unnest(array[i2_11.c12, i2_12.c11, i2_12.c13, i2_11.c32, i2_12.c31, i2_12.c33, i2_21.c22, i2_22.c21, i2_22.c23]),

    unnest(array[i2_11.c21, i2_11.c23, i2_12.c22, i2_21.c11, i2_21.c13, i2_22.c12, i2_21.c31, i2_21.c33, i2_22.c32]),
    unnest(array[i2_11.c22, i2_12.c21, i2_12.c23, i2_21.c12, i2_22.c11, i2_22.c13, i2_21.c32, i2_22.c31, i2_22.c33]),

    i2_11.cx + unnest(array[0,0,0,1,1,1,2,2,2]),
    i2_11.cy + unnest(array[0,1,2,0,1,2,0,1,2])

  from i2 as i2_11
  inner join i2 as i2_12 on (i2_11.cx, i2_11.cy) = (i2_12.cx, i2_12.cy - 1)
  inner join i2 as i2_21 on (i2_11.cx, i2_11.cy) = (i2_21.cx - 1, i2_21.cy)
  inner join i2 as i2_22 on (i2_11.cx, i2_11.cy) = (i2_22.cx - 1, i2_22.cy - 1)

  where i2_11.cx % 2 = 0 and i2_11.cy % 2 = 0
),
-- block size 2x2, block count 3x3, cell count 6x6
i3 (c11, c12, c13, c21, c22, c23, c31, c32, c33, cx, cy) as (
  select
    out11, out12, out13,
    out21, out22, out23,
    out31, out32, out33,
    cx, cy
  from i2_split
  inner join rule_2 on (c11, c12, c21, c22) = (in11, in12, in21, in22)
),
-- block size 3x3, block count 3x3, cell count 9x9
i4 (c11, c12, c21, c22, cx, cy) as (
  select
    out11, out12,
    out21, out22,
    cx * 2 + x, cy * 2 + y
  from i3
  inner join rule_3_split on (c11, c12, c13, c21, c22, c23, c31, c32, c33) = (in11, in12, in13, in21, in22, in23, in31, in32, in33)
),
-- block size 2x2, block count 6x6, cell count 12x12
i5 (c11, c12, c13, c21, c22, c23, c31, c32, c33, cx, cy) as (
  select
    out11, out12, out13,
    out21, out22, out23,
    out31, out32, out33,
    cx, cy
  from i4
  inner join rule_2 on (c11, c12, c21, c22) = (in11, in12, in21, in22)
)
-- block size 3x3, block count 6x6, cell count 18x18

select sum(
  c11::integer + c12::integer + c13::integer +
  c21::integer + c22::integer + c23::integer +
  c31::integer + c32::integer + c33::integer
) from i5;

rollback;
