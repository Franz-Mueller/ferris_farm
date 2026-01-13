create table
    "user" (
        user_id uuid primary key default uuid_generate_v1mc (),
        username text collate "case_insensitive" unique not null,
        -- Email should be neither unique nor required; 
        -- it's optional for users who want to receive important messages by email.
        email text collate "case_insensitive",
        password_hash text not null,
        created_at timestamptz not null default now (),
        updated_at timestamptz
    );

select
    trigger_updated_at ('"user"');