# RCMS

> Name Suggestions Appreciated

## Project Description

RustCMS is a barebones backend CMS that works much like [Processwire](https://processwire.com/) works. This is not a markdown site generator like other CMSs. If you would like something of that nature, see [here](#repositories-like-this).

## Project State

Version: Pre-alpha

|             | Ready |
| ----------- | ----------- |
| Backend | ✅ |
| Frontend | ✅ |
| Production | ❌ |

This project has a functioning frontend, but it is not fully featured yet.

## Note on testing

Whenever you run the tests, it is best to have a clean database.

You **MUST** run it using this command (since mocking for Diesel isn't mature yet):

`cargo test -- --test-thread=1`

## Dev Environment Setup

Required items:
* Rustc
* MySQL (8.0+ preferred)
* Diesel CLI (`cargo install diesel_cli --no-default-features --features mysql`)

1. Clone this repository.

`git clone git@github.com:SpBills/rust-cms.git`

Then, enter the repository.

`cd rust-cms`

2. Create a MySQL database.

Log in to your MySQL server, and `CREATE DATABASE rustcms;`

3. Add a user to this database and give them privileges.

Change anything labelled `rustcms`, as it is a placeholder. Since this is a dev environment, you shouldn't have to worry about security too much though.

`CREATE USER 'rustcms'@'%' IDENTIFIED BY 'rustcms';`

Giving them privileges:

`GRANT ALL PRIVILEGES ON rustcms.* TO 'rustcms'@'%';`

`FLUSH PRIVILEGES`

4. Setup environment variables.

Since this is a development environment, we won't be explicitly setting these on the system. Instead, we'll store them in a file named `.env` in the root of this project.

First, create a file named `.env`.

Next, put in your MySQL connection string. Extensive examples for a full .env can be seen [here](#environment-variables).

The MySQL connection string is not used for the program connecting to the database (this is done in rcms.json), but rather for running the migrations.

`DATABASE_URL=mysql://rustcms:rustcms@localhost:3306/rustcms`

5. Run Diesel migrations.

First, install Diesel.

`cargo install diesel_cli --no-default-features --features mysql`

Next, run the migrations.

`diesel migration run`

## Environment Variables
Most all environment setup will be handled by an installer GUI in the future. For now, we put all "environment" variables into `rcms.json`.

## rcms.json examples

```json
{
    "mysql_username": String,
    "mysql_password": String,
    "mysql_url": String,
    "mysql_port": Number,
    "mysql_database": String,
    "bind_address": String,
    "bind_port": Number
}
```

## Notes on 404 Pages

404s are handled (currently) by creating a file called `404.html.` It will automatically be added as your 404 page.

## Repositories Like This

Markdown static site generators:

(Data retrieved from [here](https://www.arewewebyet.org/topics/cms/))

https://github.com/getzola/zola

https://github.com/cobalt-org/cobalt.rs

https://github.com/rust-lang/mdBook
