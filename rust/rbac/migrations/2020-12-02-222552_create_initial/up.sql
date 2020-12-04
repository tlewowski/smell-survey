-- Your SQL goes here

create table users(
  id bigserial primary key
);

create table teams(
  id bigserial primary key,
  parent_team_id bigint,
  constraint parent_team_id_fkey foreign key (parent_team_id)
    references teams(id) match simple
    on update cascade on delete cascade
);

create table roles(
  id bigserial primary key,
  name text not null
);

create table team_members(
  id bigserial primary key,
  user_id bigint not null,
  team_id bigint not null,
  constraint user_id_fkey foreign key(user_id)
    references users(id) match simple
    on update cascade on delete cascade,
  constraint team_id_fkey foreign key(team_id)
    references teams(id) match simple
    on update cascade on delete cascade
);


create table user_system_roles(
  id bigserial primary key,
  role_id bigint not null,
  user_id bigint not null,
  constraint role_id_fkey foreign key(role_id)
    references roles(id) match simple
    on update cascade on delete cascade,
  constraint user_id_fkey foreign key(user_id)
    references users(id) match simple
    on update cascade on delete cascade
);

create table team_member_roles(
  id bigserial primary key,
  member_id bigint not null,
  role_id bigint not null,
  constraint member_id_fkey foreign key(member_id)
    references team_members(id) match simple
    on update cascade on delete cascade,
  constraint role_id_fkey foreign key(role_id)
    references roles(id) match simple
    on update cascade on delete cascade
);

create table audit_trail(
  id bigserial primary key,
  unix_timestamp bigint not null,
  actor_id bigint not null,
  action_content_stringified text not null,
  was_successful boolean default false,
  constraint actor_id_fkey foreign key(actor_id)
    references users(id) match simple
    on update cascade on delete cascade
);

insert into roles(name) values
('owner'),('admin'),('manager'),
('member'),('researcher'),('reviewer'),
('guest');
