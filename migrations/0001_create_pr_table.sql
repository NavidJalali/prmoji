create table if not exists pull_requests (
    id uuid primary key,
    url text not null,
    inserted_at timestamptz not null,
    channel varchar(127) not null,
    timestamp varchar(127) not null
);

create index if not exists pull_requests_url_idx on pull_requests(url);
