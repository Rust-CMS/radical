CREATE TABLE pages (
    pageId int AUTO_INCREMENT PRIMARY KEY,
    title varchar(500),
)

CREATE TABLE modules (
    module_type_id int,
    pageId int,
    FOREIGN KEY (pageId) REFERENCES pages(pageId)
)

CREATE TABLE module_types (
    module_type_id int AUTO_INCREMENT PRIMARY KEY,
    title varchar(500),
    desc varchar(500)
)