CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table if not exists worker
(
    id   uuid UNIQUE DEFAULT uuid_generate_v4(),
    name  TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS tasks
(
    task   uuid NOT NULL DEFAULT uuid_generate_v4() unique,
    text   TEXT not null UNIQUE,
    worker uuid NOT NULL,
    foreign key (worker) references worker(id)

);
create table if not exists manager
(
    id       uuid DEFAULT uuid_generate_v4() primary key,
    password TEXT not null,
    name     TEXT
);



