table! {
    module_types (module_type_id) {
        module_type_id -> Integer,
        title -> Varchar,
        module_desc -> Varchar,
    }
}

table! {
    modules (module_id) {
        module_id -> Integer,
        module_type_id -> Integer,
        title -> Varchar,
        page_url -> Varchar,
        content -> Text,
    }
}

table! {
    pages (url_path) {
        url_path -> Varchar,
        title -> Varchar,
        time_created -> Timestamp,
    }
}

table! {
    web_config (config_key) {
        config_key -> Varchar,
        config_val -> Varchar,
    }
}

joinable!(modules -> module_types (module_type_id));
joinable!(modules -> pages (page_url));

allow_tables_to_appear_in_same_query!(
    module_types,
    modules,
    pages,
    web_config,
);
