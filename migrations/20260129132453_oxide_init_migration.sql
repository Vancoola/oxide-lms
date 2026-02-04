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
    'associate',
    'bachelor',
    'master',
    'doctorate',
    'professional',
    'specialist'
    );

create type lesson_type as enum (
    'lecture',
    'seminar',
    'practical',
    'laboratory',
    'colloquium',
    'consultation',
    'master_class',
    'training',
    'mixed'
    );

create table if not exists faculties
(
    id uuid primary key default gen_random_uuid(),
    name varchar(255) not null,
    short_name varchar(255) not null,
    description text default null,
    is_active bool not null default false
);

create table if not exists departments(
    id uuid primary key default gen_random_uuid(),
    faculties_id uuid not null references faculties(id) on delete cascade
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
    speciality_id uuid not null references specialities(id) on delete cascade,
    code char(255) not null,
    name text default null,
    course smallint not null default 1,
    created_at timestamp with time zone default now(),
    is_active bool not null default false
);

create table if not exists profiles(
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    first_name text not null,
    last_name text not null,
    patronymic text default null,

    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now()
);

create table if not exists student_accounts(
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,

    student_id_number text default null,

    status training_status not null,

    -- думаю стоит убрать, тк студент привязан к группе, а группа к специальности
    -- speciality_id uuid not null references specialities(id) on delete cascade,
    faculty_id uuid not null references faculties(id) on delete cascade,
    group_id uuid not null references academic_groups(id) on delete cascade,

    enrolled_at timestamp with time zone default null,
    graduated_at timestamp with time zone default null
);

create table if not exists staff(
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,

    departments_id uuid not null references departments(id) on delete cascade
);

-- курс, например "физика"
create table if not exists course(
    id uuid primary key default gen_random_uuid()
);

-- дисциплина, например "механика"
create table if not exists subject(
    id uuid primary key default gen_random_uuid(),
    course_id uuid not null references course(id) on delete cascade,

    type lesson_type not null,

    audience varchar(15) not null
);

-- урок в рамках дисциплины
create table if not exists lesson(
    id uuid primary key default gen_random_uuid(),
    subject uuid not null references subject(id) on delete cascade
);

create index idx_specialities_faculty on specialities(faculty_id);
create index idx_groups_faculty on academic_groups(faculty_id);

create index idx_student_user on student_accounts(user_id);
create index idx_staff_user on staff(user_id);
