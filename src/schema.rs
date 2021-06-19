table! {
    shortlinks (id) {
        id -> Text,
        link -> Text,
        created_at -> Timestamptz,
        enabled -> Bool,
        hits -> Int8,
    }
}
