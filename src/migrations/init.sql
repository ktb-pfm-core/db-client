create schema if not exists master;
create schema if not exists record;
create schema if not exists monitoring;
create enum if not exists database_type
values
  (
    'mysql',
    'postgresql',
    'kafka',
    'mongodb',
    'redis'
  );
create table if not exists master.project (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
name varchar(100) not null unique
);create table if not exists master.db (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
project_id int8 references master.project(id),
type database_type not null,
name varchar(200) not null
) create table if not exists master.db_metric (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
db_type database_type not null,
name varchar(100) not null
) create table if not exists master.service (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
project_id int8 not null references master.project(id),
name varchar(100) not null,
unique(project_id, name)
);create table if not exists master.flow (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
project_id int8 not null references master.project(id),
service_id int8 not null references master.s(id),
name varchar(200) not null,
unique(project_id, service_id, name)
);create table if not exists master.project_release (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
project_id int8 not null references master.project(id),
name varchar(200) not null,
unique(project_id, name)
);create table if not exists master.execution_type (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
name varchar(20) not null unique
)
insert into
master.execution_type (name)
values
('capacity'),
('scaling'),
('e2e-load'),
('e2e-external-load');create table if not exists record.flow_level (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
release_id int8 not null references master.project_release(id),
flow_id int8 not null references master.flow(id),
execution_type_id int8 not null references master.execution_type(id),
resource_map jsonb,
replica_map jsonb,
is_pipeline bool not null,
timestamp timestamp not null
);create table if not exists record.api_level (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
flow_level_id int8 not null references record.flow_level(id),
tag varchar(20),
vu int2 not null,
duration varchar(10) not null,
tps numeric(10, 2) not null,
error_rate numeric(10, 2) not null,
rt_avg numeric(10, 2) not null,
rt_min numeric(10, 2) not null,
rt_max numeric(10, 2) not null,
rt_p90 numeric(10, 2) not null,
rt_p95 numeric(10, 2) not null,
rt_p99 numeric(10, 2) not null,
is_cpu_below_request bool,
timestamp timestamp not null,
start_time timestamp not null,
end_time timestamp not null,
);create table if not exists monitoring.service (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
api_level_id int8 not null references record.api_level(id),
cpu_utilization numeric(10, 2) [],
cpu_request numeric(10, 2) [],
cpu_limit numeric(10, 2) [],
memory_utilization numeric(10, 2) [],
memory_request numeric(10, 2) [],
memory_limit numeric(10, 2) []
);create table if not exists monitoring.db (
id int8 not null primary key GENERATED ALWAYS AS IDENTITY,
db_id int8 not null references master.db(id),
metric_id int8 not null references master.db_metric(id),
api_level_id int8 not null references record.api_level(id),
values
    numeric(10, 2) []
);
create view record.report AS
select
  pr."name" as release,
  e."name" as execution_type,
  p.name AS project,
  s.name AS service,
  f.name AS flow,
  fl.id as flow_level_id,
  al.tag,
  al.vu,
  al.duration,
  al.tps,
  al.error_rate,
  al.rt_avg,
  al.rt_min,
  al.rt_max,
  al.rt_p90,
  al.rt_p95,
  al.rt_p99,
  al.is_cpu_below_request,
  fl.resource_map,
  fl.replica_map,
  ms.cpu_utilization,
  ms.cpu_request,
  ms.cpu_limit,
  ms.memory_utilization,
  ms.memory_request,
  ms.memory_limit,
  al."timestamp",
  al.start_time,
  al.end_time
from
  record.api_level as al
  inner join monitoring.service ms on al.id = ms.id
  inner join record.flow_level AS fl on al.flow_level_id = fl.id
  INNER JOIN master.flow AS f ON fl.flow_id = f.id
  INNER JOIN master.service AS s ON f.service_id = s.id
  INNER JOIN master.project AS p ON s.project_id = p.id
  inner join master.execution_type as e on e.id = fl.execution_type_id
  inner join master.project_release as pr on pr.id = fl.release_id;
