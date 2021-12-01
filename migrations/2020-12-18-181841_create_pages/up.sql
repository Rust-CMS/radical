CREATE TABLE IF NOT EXISTS pages (
    uuid varchar(255) PRIMARY KEY,
    page_name varchar(500) NOT NULL,
    page_url varchar(255) NOT NULL,
    page_title varchar(500) NOT NULL,
    time_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT IGNORE INTO pages (page_name, uuid, page_url, page_title) VALUES ("index", (SELECT UUID()), "/", "Home");

CREATE TABLE module_category (
    uuid varchar(255) PRIMARY KEY,
    page_uuid varchar(255) NOT NULL,
    title varchar(255) NOT NULL,
    FOREIGN KEY (page_uuid) REFERENCES pages(uuid) ON DELETE CASCADE
);

insert ignore into module_category (uuid, page_uuid, title) VALUES ((SELECT UUID()), (SELECT uuid FROM pages LIMIT 1), "colors");

CREATE TABLE IF NOT EXISTS modules (
    uuid varchar(255) PRIMARY KEY,
    page_uuid VARCHAR(255) NOT NULL,
    category_uuid VARCHAR(255),
    title varchar(255) NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (page_uuid) REFERENCES pages(uuid) ON DELETE CASCADE,
    FOREIGN KEY (category_uuid) REFERENCES module_category(uuid) ON DELETE CASCADE
);

INSERT IGNORE INTO modules (uuid, title, page_uuid, content) VALUES ((SELECT UUID()), "title", (SELECT uuid FROM pages LIMIT 1), "Welcome to Radical.");
INSERT IGNORE INTO modules (uuid, title, page_uuid, content) VALUES ((SELECT UUID()), "small", (SELECT uuid FROM pages LIMIT 1), "A Rusty Wordpress Replacement");
INSERT IGNORE INTO modules (uuid, title, page_uuid, content) VALUES ((SELECT UUID()), "githublink", (SELECT uuid FROM pages LIMIT 1), "https://github.com/Rust-CMS/radical");
INSERT IGNORE INTO modules (uuid, title, page_uuid, content) VALUES ((SELECT UUID()), "githublink_tooling", (SELECT uuid FROM pages LIMIT 1), "https://github.com/Rust-CMS/tooling");

INSERT IGNORE INTO modules (uuid, title, page_uuid, content, category_uuid) VALUES ((SELECT UUID()), "color1", (SELECT uuid FROM pages LIMIT 1), "red", (SELECT uuid FROM module_category LIMIT 1));
INSERT IGNORE INTO modules (uuid, title, page_uuid, content, category_uuid) VALUES ((SELECT UUID()), "color2", (SELECT uuid FROM pages LIMIT 1), "blue", (SELECT uuid FROM module_category LIMIT 1));
INSERT IGNORE INTO modules (uuid, title, page_uuid, content, category_uuid) VALUES ((SELECT UUID()), "color3", (SELECT uuid FROM pages LIMIT 1), "green", (SELECT uuid from module_category LIMIT 1));

CREATE TABLE IF NOT EXISTS users (
    uuid varchar(255) PRIMARY KEY,
    username varchar(255) NOT NULL UNIQUE,
    password varchar(255) NOT NULL,
    token varchar(511)
);

INSERT IGNORE INTO users (uuid, username, password) VALUES ((SELECT UUID()), "root", "");