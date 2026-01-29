-- Add migration script here
create table if not exists users(
    id uuid primary key default gen_random_uuid(),
    email text not null,
    password text not null,
    name text not null
);

create index if not exists idx_users_email on users(email);

create type course_status as enum('draft', 'closed', 'active', 'archived');

create table if not exists course(
    id uuid primary key default gen_random_uuid(),
    name text not null,
    description text not null,
    status course_status default 'draft',
    started_at timestamptz,
    ended_at timestamptz
);

