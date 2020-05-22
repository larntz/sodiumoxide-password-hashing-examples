DROP TABLE IF EXISTS users;
CREATE TABLE IF NOT EXISTS users
(
    id uuid NOT NULL DEFAULT uuid_generate_v1mc(),
    user_name varchar(128) UNIQUE NOT NULL,
	password_hash_bin bytea NOT NULL,
	password_hash_char varchar(128) NOT NULL,
    email_address varchar(128) NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)

);
