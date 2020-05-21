# sodiumoxide password hashing examples

This repo was created to go with a blog post I wrote about using
sodiumoxide for storing and verifying password hashes.

When I started playing with it I didn't find a lot of examples or
explicit information on how to use the library so I decided
to do some research and attempt to make an ELI5 type post.

You can read the post here: [https://blue42.net](https://blue42.net/code/rust/examples/sodiumoxide-password-hashing/post/).

## database setup

To run the database examples you'll need a postgres database and to set an
environment variable with the connection string.

### Example ENV

``` shell
export DATABASE_URL=postgres://postgres:database_password@localhost/database_name
```

### Run a local postgres DB via docker

If you don't have a db available, but do have docker you can run a small container
locally for testing.

``` bash
## create the db and container
## !! don't forget to change the postgres_user_password !!

docker run -d --name pgsql12 \
        -e POSTGRES_PASSWORD=postgres_user_password \
        -p 5432:5432 \
        postgres:12-alpine

## to start or stop the db in the future run

docker start pgsql12

# or

docker stop pgsql12
```
