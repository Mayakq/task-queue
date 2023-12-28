CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS tasks
(
    id     uuid          DEFAULT uuid_generate_v4() primary key unique,
    task   uuid NOT NULL DEFAULT uuid_generate_v4() unique,
    text   TEXT not null UNIQUE ,
    worker uuid NOT NULL
);
create table if not exists manager
(
    id       uuid DEFAULT uuid_generate_v4() primary key,
    password TEXT not null,
    name     TEXT
);
create table if not exists worker
(
    id   uuid DEFAULT uuid_generate_v4(),
    task uuid NOT NULL default uuid_generate_v4(),
    name TEXT,
    foreign key (task) references tasks (task)
);

insert into tasks (text, worker)
values ('god', (select id from manager where name = '123'))
