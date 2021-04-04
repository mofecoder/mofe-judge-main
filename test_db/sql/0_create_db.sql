create database if not exists p6jav_cafecoder_prod;

use p6jav_cafecoder_prod;

create table ar_internal_metadata
(
    `key`      varchar(255) not null
        primary key,
    value      varchar(255) null,
    created_at datetime(6)  not null,
    updated_at datetime(6)  not null
);

create table clarifications
(
    id         bigint auto_increment
        primary key,
    contest_id bigint               not null,
    problem_id bigint               null,
    user_id    bigint               not null,
    question   varchar(255)         not null,
    answer     varchar(255)         null,
    publish    tinyint(1) default 0 not null,
    created_at datetime(6)          not null,
    updated_at datetime(6)          not null,
    deleted_at datetime             null
);

create index index_clarifications_on_contest_id
    on clarifications (contest_id);

create index index_clarifications_on_problem_id
    on clarifications (problem_id);

create index index_clarifications_on_user_id
    on clarifications (user_id);

create table contests
(
    id           bigint auto_increment
        primary key,
    slug         varchar(255)  not null,
    name         varchar(255)  not null,
    description  varchar(4096) null,
    penalty_time int default 0 not null,
    start_at     datetime      null,
    end_at       datetime      null,
    created_at   datetime(6)   not null,
    updated_at   datetime(6)   not null,
    deleted_at   datetime      null,
    constraint index_contests_on_slug
        unique (slug)
);

create table registrations
(
    id         bigint auto_increment
        primary key,
    user_id    bigint      not null,
    contest_id bigint      not null,
    created_at datetime(6) not null,
    updated_at datetime(6) not null,
    deleted_at datetime    null
);

create index index_registrations_on_contest_id
    on registrations (contest_id);

create index index_registrations_on_user_id
    on registrations (user_id);

create table schema_migrations
(
    version varchar(255) not null
        primary key
);

create table testcase_results
(
    id               bigint auto_increment
        primary key,
    submit_id        bigint      not null,
    testcase_id      bigint      not null,
    status           varchar(16) not null,
    execution_time   int         not null,
    execution_memory int         not null,
    created_at       datetime(6) not null,
    updated_at       datetime(6) not null,
    deleted_at       datetime    null
);

create index index_testcase_results_on_submit_id
    on testcase_results (submit_id);

create index index_testcase_results_on_testcase_id
    on testcase_results (testcase_id);

create table users
(
    id                     bigint auto_increment
        primary key,
    provider               varchar(255) default 'email'  not null,
    uid                    varchar(255) default ''       not null,
    encrypted_password     varchar(255) default ''       not null,
    reset_password_token   varchar(255)                  null,
    reset_password_sent_at datetime                      null,
    allow_password_change  tinyint(1)   default 0        null,
    remember_created_at    datetime                      null,
    sign_in_count          int          default 0        not null,
    current_sign_in_at     datetime                      null,
    last_sign_in_at        datetime                      null,
    current_sign_in_ip     varchar(255)                  null,
    last_sign_in_ip        varchar(255)                  null,
    confirmation_token     varchar(255)                  null,
    confirmed_at           datetime                      null,
    confirmation_sent_at   datetime                      null,
    unconfirmed_email      varchar(255)                  null,
    role                   varchar(255) default 'member' not null,
    name                   varchar(255)                  null,
    atcoder_id             varchar(16)                   null,
    atcoder_rating         int                           null,
    email                  varchar(255)                  null,
    tokens                 text                          null,
    created_at             datetime(6)                   not null,
    updated_at             datetime(6)                   not null,
    deleted_at             datetime                      null,
    constraint index_users_on_confirmation_token
        unique (confirmation_token),
    constraint index_users_on_email
        unique (email),
    constraint index_users_on_name
        unique (name),
    constraint index_users_on_reset_password_token
        unique (reset_password_token),
    constraint index_users_on_uid_and_provider
        unique (uid, provider)
);

create table problems
(
    id             bigint auto_increment
        primary key,
    slug           varchar(255)     null,
    name           varchar(255)     null,
    contest_id     bigint           null,
    writer_user_id bigint default 1 not null,
    checker_path   varchar(255)     not null,
    position       varchar(4)       null,
    uuid           varchar(255)     null,
    difficulty     varchar(16)      not null,
    statement      varchar(4096)    not null,
    constraints    varchar(2048)    not null,
    input_format   varchar(1024)    not null,
    output_format  varchar(1024)    not null,
    created_at     datetime(6)      not null,
    updated_at     datetime(6)      not null,
    deleted_at     datetime         null,
    constraint index_problems_on_slug
        unique (slug),
    constraint fk_rails_82fc22963d
        foreign key (writer_user_id) references users (id),
    constraint fk_rails_ee26900320
        foreign key (contest_id) references contests (id)
);

create index index_problems_on_contest_id
    on problems (contest_id);

create index index_problems_on_writer_user_id
    on problems (writer_user_id);

create table submits
(
    id               bigint auto_increment
        primary key,
    user_id          int          not null,
    problem_id       bigint       not null,
    path             varchar(255) not null,
    status           varchar(16)  not null,
    point            int          null,
    execution_time   int          null,
    execution_memory int          null,
    compile_error    text         null,
    lang             varchar(255) not null,
    created_at       datetime(6)  not null,
    updated_at       datetime(6)  not null,
    deleted_at       datetime     null,
    constraint fk_rails_0ac3e2f6fc
        foreign key (problem_id) references problems (id)
);

create index index_submits_on_problem_id
    on submits (problem_id);

create table testcase_sets
(
    id         bigint auto_increment
        primary key,
    problem_id bigint       not null,
    name       varchar(255) not null,
    points     int          not null,
    is_sample  tinyint(1)   not null,
    created_at datetime(6)  not null,
    updated_at datetime(6)  not null,
    deleted_at datetime     null,
    constraint fk_rails_9606b8a9e5
        foreign key (problem_id) references problems (id)
);

create index index_testcase_sets_on_problem_id
    on testcase_sets (problem_id);

create table testcases
(
    id          bigint auto_increment
        primary key,
    problem_id  bigint default 1 not null,
    name        varchar(255)     null,
    input       longtext         null,
    output      longtext         null,
    explanation varchar(2048)    null,
    created_at  datetime(6)      not null,
    updated_at  datetime(6)      not null,
    deleted_at  datetime         null,
    constraint fk_rails_740afe3d3a
        foreign key (problem_id) references problems (id)
);

create table testcase_testcase_sets
(
    id              bigint auto_increment
        primary key,
    testcase_id     bigint      not null,
    testcase_set_id bigint      not null,
    created_at      datetime(6) not null,
    updated_at      datetime(6) not null,
    deleted_at      datetime    null,
    constraint fk_rails_400d292f6a
        foreign key (testcase_id) references testcases (id),
    constraint fk_rails_dc4fbe8a3c
        foreign key (testcase_set_id) references testcase_sets (id)
);

create index index_testcase_testcase_sets_on_testcase_id
    on testcase_testcase_sets (testcase_id);

create index index_testcase_testcase_sets_on_testcase_set_id
    on testcase_testcase_sets (testcase_set_id);

create index index_testcases_on_problem_id
    on testcases (problem_id);

create table tester_relations
(
    id             bigint auto_increment
        primary key,
    problem_id     bigint      not null,
    tester_user_id bigint      not null,
    approved       tinyint(1)  not null,
    created_at     datetime(6) not null,
    updated_at     datetime(6) not null,
    deleted_at     datetime    null,
    constraint fk_rails_0b6cb25bec
        foreign key (tester_user_id) references users (id),
    constraint fk_rails_34f1f29dce
        foreign key (problem_id) references problems (id)
);

create index index_tester_relations_on_problem_id
    on tester_relations (problem_id);

create index index_tester_relations_on_tester_user_id
    on tester_relations (tester_user_id);

