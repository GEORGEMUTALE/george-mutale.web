create table hello(
    id varchar not null,
    title varchar not null
);
create unique index book_id on hello(id);