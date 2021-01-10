CREATE TABLE pages (
    page_id int AUTO_INCREMENT PRIMARY KEY NOT NULL,
    title varchar(500) NOT NULL,
    time_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE module_types (
    module_type_id int AUTO_INCREMENT PRIMARY KEY NOT NULL,
    title varchar(500) NOT NULL,
    module_desc varchar(500) NOT NULL
);

/* Doing each in a separate statement since it's more readable. */
INSERT INTO module_types (title, module_desc) VALUES ('paragraph', 'A paragraph module for general text.');
INSERT INTO module_types (title, module_desc) VALUES ('header', 'A header module for displaying things in large text.');
INSERT INTO module_types (title, module_desc) VALUES ('image', 'Allows for inserting images into the page.');

CREATE TABLE modules (
    module_id int AUTO_INCREMENT PRIMARY KEY NOT NULL,
    module_type_id int NOT NULL,
    page_id int NOT NULL,
    content TEXT,
    FOREIGN KEY (page_id) REFERENCES pages(page_id) ON DELETE CASCADE,
    FOREIGN KEY (module_type_id) REFERENCES module_types(module_type_id) ON DELETE CASCADE
);

CREATE TABLE web_config (
    config_key VARCHAR(100) PRIMARY KEY NOT NULL,
    config_val VARCHAR(100) NOT NULL
);

INSERT INTO web_config (config_key, config_val) VALUES ("setup", "start");