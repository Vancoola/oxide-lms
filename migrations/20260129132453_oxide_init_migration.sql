-- Add migration script here
create table if not exists users
(
    id       uuid primary key default gen_random_uuid(),
    email    text not null,
    password text not null,
    is_admin bool not null    default false
);

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

-- create type organizational_units_type as enum (
--     'university',
--     'college',
--     'faculty',
--     'school',
--     'institute',
--     'department',
--     'research_center',
--     'academic_group',
--     'division'
--     );

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

create type course_enrollments_status as enum (
    'enrolled',
    'completed',
    'dropped'
    );

create table if not exists organizational_unit_types
(
    id                uuid primary key default gen_random_uuid(),
    code              text unique  not null, -- university, faculty, group
    name              varchar(255) not null, -- {"en": "Department", "ru": "Кафедра"}
    can_have_students bool             default false
);

create table if not exists organizational_units
(
    id                uuid primary key                                        default gen_random_uuid(),
    name              varchar(255)                                   not null,
    short_name        varchar(100),
    parent_id         uuid references organizational_units (id),
    specific_metadata jsonb,
    is_active         boolean                                                 default true,
    created_at        timestamptz                                             default now(),
    type_id           uuid references organizational_unit_types (id) not null,
    path              uuid[]                                         not null default '{}'
);

create table if not exists specialities
(
    id                     uuid primary key     default gen_random_uuid(),
    organizational_unit_id uuid references organizational_units (id),
    code                   varchar(25) not null,
    name                   text        not null,
    degree                 degree      not null,
    is_active              bool        not null default false
);

create table if not exists profiles
(
    id          uuid primary key                  default gen_random_uuid(),
    user_id     uuid                     not null references users (id) on delete cascade,
    first_name  text                              default null,
    last_name   text                              default null,
    middle_name text                              default null,

    is_active   bool                     not null default false,

    created_at  timestamp with time zone not null default now(),
    updated_at  timestamp with time zone not null default now()
);

create table if not exists student_accounts
(
    id                     uuid primary key         default gen_random_uuid(),
    user_id                uuid            not null references users (id) on delete cascade,

    student_id_number      text                     default null,

    status                 training_status not null,

    organizational_unit_id uuid            not null references organizational_units (id), -- Привязка к организации, например к Department

    enrolled_at            timestamp with time zone default null,
    graduated_at           timestamp with time zone default null
);

create table if not exists staff
(
    id                     uuid primary key default gen_random_uuid(),
    user_id                uuid not null references users (id) on delete cascade,

    organizational_unit_id uuid references organizational_units (id)
);

create table if not exists educational_programs
(
    id                     uuid primary key,
    name                   varchar(255) not null,
    code                   varchar(50)  not null,
    degree                 degree       not null, -- bachelor, master и т.д.

    -- Программа привязана к организационной единице
    organizational_unit_id uuid references organizational_units (id),

    -- Учебный план
    duration_years         integer,               -- 4 года, 2 года и т.д.
    total_credits          integer,               -- 240 ECTS, 120 кредитов и т.д.
    description            text
);


-- курс, например "физика"
create table if not exists courses
(
    id       uuid primary key,
    name     varchar(255) not null,
    code     varchar(50),   -- "CS101", "PHY-201" и т.д.

    -- Часть программы
    semester integer,       -- 1-й семестр, 2-й семестр
    credits  decimal(4, 1), -- 3.0, 4.5 кредита

    audience varchar(15)
);

create table if not exists program_courses
(
    id               uuid primary key default gen_random_uuid(),
    program_id       uuid not null references educational_programs (id) on delete cascade,
    course_id        uuid not null references courses (id) on delete cascade,

    is_compulsory    bool             DEFAULT false, -- обязательный / по выбору
    credits_override decimal(4, 1),                  -- если кредиты отличаются от базовых в course
    recommended_year smallint,                       -- 1-й курс, 2-й и т.д.

    created_at       timestamptz      DEFAULT now(),
    created_by       uuid REFERENCES users (id),

    UNIQUE (program_id, course_id)
);

create table if not exists academic_terms
(
    id         uuid primary key default gen_random_uuid(),
    name       text not null, -- e.g., Fall 2025
    start_date date not null,
    end_date   date not null
);

create table if not exists lesson_series
(
    id                uuid primary key     default gen_random_uuid(),
    course_id         uuid        not null references courses (id),
    teacher_id        uuid references staff (id),
    lesson_type       lesson_type not null,
    classroom         varchar(50),
    topic_pattern     varchar(255),                   -- шаблон темы, можно с плейсхолдерами
    materials_pattern jsonb,                          -- общие материалы

    -- Расписание
    term_id           uuid        not null references academic_terms (id),
    weekday           smallint    not null,           -- 0 = Sunday, 1 = Monday, ..., 6 = Saturday
    start_time        time        not null,           -- 10:00:00
    duration_minutes  integer     not null default 90,
    interval_weeks    smallint             default 1, -- 1 = каждую неделю, 2 = через неделю

    -- Границы
    first_lesson_date date        not null,           -- дата первого занятия
    last_lesson_date  date,                           -- может быть NULL = до конца семестра

    -- Дополнительно
    created_at        timestamptz          default now(),
    created_by        uuid references staff (id),
    is_active         bool                 default true
);

-- дисциплина, например "механика"
create table if not exists lessons
(
    id                    uuid primary key,
    course_id             uuid references courses (id),

    -- Когда и где
    scheduled_at          timestamptz not null,
    duration_minutes      integer     not null,
    classroom             varchar(50),

    -- Кто ведет
    teacher_id            uuid references staff (id),

    -- Тема занятия
    topic                 varchar(255),
    materials             jsonb,       -- ссылки на материалы

    -- Тип урока
    lesson_type           lesson_type, -- lecture, seminar и т.д.

    series_id             uuid        references lesson_series (id) on delete set null,
    is_cancelled          bool default false,
    is_rescheduled        bool default false,
    original_scheduled_at timestamptz
);



create table if not exists course_enrollments
(
    id         uuid primary key default gen_random_uuid(),
    student_id uuid references student_accounts (id),
    course_id  uuid references courses (id),
    term_id    uuid references academic_terms (id),
    grade      decimal(3, 2),
    status     course_enrollments_status not null
);

create table if not exists assignment_templates
(
    id                uuid primary key default gen_random_uuid(),
    course_id         uuid references courses (id) not null,
    related_lesson_id uuid                         references lessons (id) on delete set null,
    title             text                         not null,
    description       text,
    max_points        integer          default 100
);

create table if not exists published_assignments
(
    id          uuid primary key default gen_random_uuid(),
    template_id uuid references assignment_templates (id) not null,
    term_id     uuid references academic_terms (id)       not null,
    due_date    timestamptz,
    weight      decimal(3, 2)    default 0.1
);

create table if not exists assignment_submissions
(
    id            uuid primary key default gen_random_uuid(),
    assignment_id uuid not null references published_assignments (id),
    student_id    uuid not null references student_accounts (id),
    content       jsonb,
    submitted_at  timestamptz      default now()
);

create table if not exists grades
(
    id            uuid primary key default gen_random_uuid(),
    student_id    uuid          not null references student_accounts (id),
    course_id     uuid          not null references courses (id),

    submission_id uuid references assignment_submissions (id),
    lesson_id     uuid references lessons (id),

    score         decimal(5, 2) not null,
    feedback      text,
    graded_at     timestamptz      default now(),
    graded_by     uuid          not null references staff (id)
);


create table outbox_events
(
    id           uuid primary key     default gen_random_uuid(),
    event_type   text        not null,
    payload      jsonb       not null,
    created_at   timestamptz not null default now(),
    processed_at timestamptz,
    retry_count  int         not null default 0,
    last_error   text
);
CREATE INDEX idx_outbox_unprocessed ON outbox_events (created_at)
    WHERE processed_at IS NULL;


create unique index if not exists idx_users_email on users (email);
create unique index if not exists idx_profile_user on profiles (user_id);

create index idx_units_path_gin on organizational_units using gin (path);

create index idx_lesson_series_term on lesson_series (term_id);
create index idx_lesson_series_course on lesson_series (course_id);

create index idx_student_user on student_accounts (user_id);
create index idx_staff_user on staff (user_id);

create index idx_program_courses_program on program_courses (program_id);
create index idx_program_courses_course on program_courses (course_id);

create or replace function notify_outbox_event() returns trigger as
$$
begin
    perform pg_notify('outbox_event', new.payload::text);
    return new;
end;
$$ language plpgsql;

create trigger outbox_events_notify_trigger
    after insert or update
    on outbox_events
    for each row
execute function notify_outbox_event();

-- create or replace function update_unit_path() returns trigger as $$
--     begin
--
--     end;
-- $$ language plpgsql;


create or replace function validate_student_unit_trigger() returns trigger as
$$
begin
    if not (select can_have_students
            from organizational_unit_types t
                     join organizational_units u on u.type_id = t.id
            where u.id = new.organizational_unit_id)
    then
        raise exception 'A student cannot be assigned to this type of unit';
    end if;
    return new;
end;
$$ language plpgsql;

create trigger trg_validate_student_unit
    before insert or update
    on student_accounts
    for each row
execute function
    validate_student_unit_trigger();
