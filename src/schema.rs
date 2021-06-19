table! {
    shortlinks (id) {
        id -> Text,
        link -> Text,
        created_at -> Nullable<Timestamp>,
        enabled -> Bool,
        hits -> Nullable<Int8>,
    }
}
