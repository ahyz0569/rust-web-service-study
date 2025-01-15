-- Add migration script here
drop table if exists ezy_course_c5;

-- 테이블 생성
create table ezy_course_c5
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

insert into ezy_course_c5 (course_id, tutor_id, course_name, posted_time)
values (1, 1, 'First course', '2025-01-03 12:30:10');
insert into ezy_course_c5 (course_id, tutor_id, course_name, posted_time)
values (2, 1, 'Second course', '2025-01-03 19:48:39');