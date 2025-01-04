-- Add migration script here
-- 테이블이 존재 하면 삭제
drop table if exists ezy_course_c4;

-- 테이블 생성
create table ezy_course_c4
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
)