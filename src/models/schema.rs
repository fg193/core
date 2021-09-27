table! {
    #[sql_name="karyon_attr"]
    attrs (id) {
        id -> Int8,
        owner_entity -> Int8,
        editor_entity -> Int8,
        viewer_entity -> Int8,
        author_entity -> Int8,
        create_time -> Timestamptz,
        modify_time -> Timestamptz,
    }
}

table! {
    #[sql_name="karyon_entity"]
    entities (id) {
        id -> Int8,
        avatar_entity -> Nullable<Int8>,
        owner_entity -> Int8,
        editor_entity -> Int8,
        viewer_entity -> Int8,
        author_entity -> Int8,
        create_time -> Timestamptz,
        modify_time -> Timestamptz,
    }
}

table! {
    #[sql_name="karyon_i18n"]
    i18ns (id) {
        id -> Int8,
        entity -> Int8,
        attr -> Int8,
        lang -> Varchar,
        value -> Varchar,
    }
}

table! {
    #[sql_name="karyon_link"]
    links (id) {
        id -> Int8,
        attr -> Int8,
        src_entity -> Int8,
        dest_entity -> Int8,
        direct -> Bool,
        ref_count -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(attrs, entities, i18ns, links);
joinable!(i18ns -> entities(entity));
joinable!(i18ns -> attrs(attr));
