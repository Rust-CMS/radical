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
        page_id -> Integer,
    }
}

table! {
    pages (page_id) {
        page_id -> Integer,
        title -> Varchar,
        time_created -> Timestamp,
    }
}

joinable!(modules -> module_types (module_type_id));
joinable!(modules -> pages (page_id));

allow_tables_to_appear_in_same_query!(
    module_types,
    modules,
    pages,
);
