# RustCMS by Spencer Bills

## Project Description

RustCMS is a barebones backend CMS that works much like [Processwire](https://processwire.com/) works. This is not a markdown site generator like other CMSs. If you would like something of that nature, see [here](#repositories-like-this).

## Project State

Version: Pre-alpha

|             | Ready |
| ----------- | ----------- |
| Backend | ✅ |
| Frontend | - |
| Production | ❌ |

Currently this project has no frontend and is just a backend API.

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

`DATABASE_URL=mysql://rustcms:rustcms@localhost:3306/rustcms`

5. Run Diesel migrations.

First, install Diesel.

`cargo install diesel_cli --no-default-features --features mysql`

Next, run the migrations.

`diesel migration run`

## Environment Variables
This project utilizes DotEnv.

### Variables

DATABASE_URL (MySQL Connection string)

### DotEnv Examples

```
// .env
DATABASE_URL=mysql://rustcms:rustcms@localhost:3306/rustcms
```

## Repositories Like This



Markdown static site generators:

(Data retrieved from [here](https://www.arewewebyet.org/topics/cms/))

https://github.com/getzola/zola

https://github.com/cobalt-org/cobalt.rs

https://github.com/rust-lang/mdBook