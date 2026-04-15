// @generated automatically by Diesel CLI.

diesel::table! {
    entangled_pair (id) {
        id -> Int4,
        src_id -> Int4,
        dst_id -> Int4,
        fidelity -> Float4,
        created_at -> Int8,
        src_measured -> Nullable<Int2>,
        dst_measured -> Nullable<Int2>,
        timeout_timestamp -> Int8,
    }
}

diesel::table! {
    links (id) {
        id -> Int4,
        length -> Int8,
        attenuation -> Float4,
        error_rate -> Float4,
        src_id -> Int4,
        dst_id -> Int4,
        next_available_time -> Int8,
    }
}

diesel::table! {
    measurements (id) {
        id -> Int4,
        node_id -> Int4,
        basis -> Int4,
        measurement_id -> Int8,
        value -> Int2,
        consumed -> Bool,
    }
}

diesel::table! {
    nodes (id) {
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
        basis -> Int4,
        measurement_id -> Int8,
        value -> Int2,
        consumed -> Bool,
    }
}

diesel::joinable!(measurements -> nodes (node_id));
diesel::joinable!(pending_measurements -> nodes (node_id));

diesel::allow_tables_to_appear_in_same_query!(
    entangled_pair,
    links,
    measurements,
    nodes,
    pending_measurements,
);
