create table users (
    id SERIAL PRIMARY KEY,
    user_types integer default 0,
    name character varying(150),
    email character varying(150),
    bio text,
    is_blocked bool default false,
    blocked_reason text, 
    is_deleted bool default false,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    created_at timestamp without time zone default CURRENT_TIMESTAMP not null
);

-- independent
-- database lock

create table customers (
    cid SERIAL PRIMARY KEY,
    is_deleted bool default false,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    created_at timestamp default without time zone default CURRENT_TIMESTAMP not null
);
-- create

-- insert ; 
insert into users(name, email, bio, blocked_reason)
values ('Fred','fred@gmail.com','i love coding','');


-- read ; select * from users ; select (name, email,bio) from users;
-- select (name, email,bio) from users where id=1;

-- update 
update users set bio=' I am a web programmer' where id=1;

-- sql functions
-- lower('TEXT)

-- adding column to existing table
alter table users add column password text default '' not null;

-- show databases; \dt
\dt
-- delete ; hard and soft delete:
-- drop a table
drop table customers
-- delete row
delete from users where id= 4;

-- undo through migration
start transaction;
rollback;