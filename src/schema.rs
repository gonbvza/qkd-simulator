// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        length -> Nullable<Int8>,
        attenuation -> Nullable<Float4>,
        error -> Nullable<Float4>,
        nodea -> Nullable<Int8>,
        nodeb -> Nullable<Int8>,
        next_available_time -> Nullable<Int8>,
    }
}

diesel::table! {
    measurements (id) {
        id -> Int4,
        node_id -> Int4,
        measurement_id -> Nullable<Int8>,
        value -> Nullable<Int2>,
        consumed -> Nullable<Bool>,
    }
}

diesel::table! {
    node (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        in_use -> Bool,
        measurements -> Int8,
        #[max_length = 255]
        node_type -> Varchar,
    }
}

diesel::table! {
    pending_measurements (id) {
        id -> Int4,
        node_id -> Int4,
        measurement_id -> Nullable<Int8>,
        value -> Nullable<Int2>,
        consumed -> Nullable<Bool>,
    }
}

diesel::joinable!(measurements -> node (node_id));
diesel::joinable!(pending_measurements -> node (node_id));

diesel::allow_tables_to_appear_in_same_query!(links, measurements, node, pending_measurements,);
