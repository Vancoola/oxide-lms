-- Add migration script here
create table if not exists users(
    id uuid primary key default gen_random_uuid(),
    email text not null,
    password text not null,
    name text not null
);

create index if not exists idx_users_email on users(email);

-- create type course_status as enum('draft', 'closed', 'active', 'archived');
--
-- create table if not exists course(
--     id uuid primary key default gen_random_uuid(),
--     name text not null,
--     description text not null,
--     status course_status default 'draft',
--     started_at timestamptz,
--     ended_at timestamptz
-- );


create type training_status as enum (
    'applicant',
    'enrolled',
    'studying',
    'academic_leave',
    'graduate',
    'expelled',
    'transferred'
    );

create type degree as enum (
    'bachelor',
    'specialist',
    'master',
    'postgraduate'
    );

create table if not exists faculties
(
    id uuid primary key default gen_random_uuid(),
    name varchar(255) not null,
    short_name varchar(255) not null,
    is_active bool not null default false
);

create table if not exists specialities(
    id uuid primary key default gen_random_uuid(),
    faculty_id uuid not null references faculties (id) on delete cascade,
    code char(25) not null,
    name text not null,
    degree degree not null,
    is_active bool not null default false
);

create table if not exists academic_groups(
    id uuid primary key default gen_random_uuid(),
    faculty_id uuid not null references faculties(id) on delete cascade,
    code char(255) not null,
    name text default null,
    course smallint not null default 1,
    speciality_id uuid not null references specialities(id) on delete cascade,
    created_at timestamp with time zone default now(),
    is_active bool not null default false
);

create table if not exists profiles(
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    first_name text not null,
    last_name text not null,
    patronymic text not null,

    student_id_number text default null,

    status training_status not null,

    speciality_id uuid not null references specialities(id) on delete cascade,
    faculty_id uuid not null references faculties(id) on delete cascade,
    group_id uuid not null references academic_groups(id) on delete cascade,

    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),
    enrolled_at timestamp with time zone default null,
    graduated_at timestamp with time zone default null
);

CREATE INDEX idx_specialities_faculty ON specialities(faculty_id);
CREATE INDEX idx_groups_faculty ON academic_groups(faculty_id);
