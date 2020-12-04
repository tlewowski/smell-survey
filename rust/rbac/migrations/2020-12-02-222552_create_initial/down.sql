-- This file should undo anything in `up.sql`

drop table if exists users;
drop table if exists teams;
drop table if exists roles;
drop table if exists team_members;
drop table if exists user_system_roles;
drop table if exists team_member_roles;
drop table if exists audit_trail;