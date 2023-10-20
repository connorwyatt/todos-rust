create table public.todos
(
    id           varchar(36) primary key,
    title        varchar(100),
    added_at     timestamptz,
    is_complete  bool,
    completed_at timestamptz
);
