DROP TABLE IF EXISTS public.users;
CREATE TABLE IF NOT EXISTS public.users
(
    id SERIAL PRIMARY KEY,
    user_name varchar(128) UNIQUE NOT NULL,
	password_hash_bin bytea,
	password_hash_char varchar(128),
    email_address varchar(128) NOT NULL
);
