CREATE TABLE IF NOT EXISTS pages (
    id int NOT NULL PRIMARY KEY AUTO_INCREMENT,
    uuid varchar(100) NOT NULL UNIQUE,
    page_name varchar(500) NOT NULL,
    page_url varchar(100) NOT NULL,
    page_title varchar(500) NOT NULL,
    time_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT IGNORE INTO pages (page_name, uuid, page_url, page_title) VALUES ("index",(SELECT UUID()), "/", "Hello world.");

CREATE TABLE IF NOT EXISTS module_types (
    module_type_id int AUTO_INCREMENT PRIMARY KEY NOT NULL,
    uuid varchar(100) NOT NULL UNIQUE,
    title varchar(500) NOT NULL,
    module_desc varchar(500) NOT NULL
);

/* Doing each in a separate statement since it's more readable. */
/* Currently these types are not used for anything and all default to 'paragraph'. */
INSERT IGNORE INTO module_types (title, uuid, module_desc) VALUES ('paragraph', (SELECT UUID()), 'A paragraph module for general text.');
INSERT IGNORE INTO module_types (title, uuid, module_desc) VALUES ('header', (SELECT UUID()), 'A header module for displaying things in large text.');
INSERT IGNORE INTO module_types (title, uuid, module_desc) VALUES ('image', (SELECT UUID()), 'Allows for inserting images into the page.');

CREATE TABLE module_category (
    id int AUTO_INCREMENT PRIMARY KEY NOT NULL,
    uuid varchar(100) NOT NULL UNIQUE,
    title varchar(100) NOT NULL
);

insert ignore into module_category (uuid, title) VALUES ((SELECT UUID()), "colors");

CREATE TABLE IF NOT EXISTS modules (
    id int AUTO_INCREMENT PRIMARY KEY NOT NULL,
    uuid varchar(100) NOT NULL UNIQUE,
    module_type_id int NOT NULL,
    title varchar(100) NOT NULL,
    page_id int NOT NULL,
    page_uuid VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    category int,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE,
    FOREIGN KEY (module_type_id) REFERENCES module_types(module_type_id) ON DELETE CASCADE,
    FOREIGN KEY (category) REFERENCES module_category(id) ON DELETE CASCADE
);

INSERT IGNORE INTO modules (module_type_id, uuid, title, page_id, page_uuid, content) VALUES (1, (SELECT UUID()), "title", 1, (SELECT uuid FROM pages WHERE id = 1), "This is the `title` module!!");
INSERT IGNORE INTO modules (module_type_id, uuid, title, page_id, page_uuid, content) VALUES (1,(SELECT UUID()), "small", 1, (SELECT uuid FROM pages WHERE id = 1), "This is the `small` module!");
INSERT IGNORE INTO modules (module_type_id, uuid, title, page_id, page_uuid, content, category) VALUES (1, (SELECT UUID()), "color1", 1, (SELECT uuid FROM pages WHERE id = 1), "red", 1);
INSERT IGNORE INTO modules (module_type_id, uuid, title, page_id, page_uuid, content, category) VALUES (1, (SELECT UUID()), "color2", 1, (SELECT uuid FROM pages WHERE id = 1), "blue", 1);
INSERT IGNORE INTO modules (module_type_id, uuid, title, page_id, page_uuid, content, category) VALUES (1, (SELECT UUID()), "color3", 1, (SELECT uuid FROM pages WHERE id = 1), "green", 1);

CREATE TABLE IF NOT EXISTS web_config (
    config_key VARCHAR(100) PRIMARY KEY NOT NULL,
    config_val VARCHAR(100) NOT NULL
);

INSERT IGNORE INTO web_config (config_key, config_val) VALUES ("setup", "start");