// @generated automatically by Diesel CLI.

diesel::table! {
    detector (id) {
        id -> Int4,
        resolution_ps -> Int8,
        cooldown_ps -> Int8,
        dark_count_rate -> Int4,
        last_detection_time -> Int8,
    }
}

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
        process_id -> Int4,
        qubit_nr -> Int4,
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
        entangled_pair_id -> Int4,
        value -> Int2,
        accepted -> Bool,
        process_id -> Int4,
    }
}

diesel::table! {
    nodes (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        locked_by -> Nullable<Int4>,
        #[max_length = 255]
        node_type -> Varchar,
        detector_id -> Int4,
    }
}

diesel::table! {
    pending_measurements (id) {
        id -> Int4,
        node_id -> Int4,
        basis -> Int4,
        measurement_id -> Int8,
        value -> Int2,
    }
}

diesel::table! {
    process (id) {
        id -> Int4,
        started_at -> Int8,
    }
}

diesel::joinable!(entangled_pair -> process (process_id));
diesel::joinable!(measurements -> entangled_pair (entangled_pair_id));
diesel::joinable!(measurements -> nodes (node_id));
diesel::joinable!(measurements -> process (process_id));
diesel::joinable!(nodes -> detector (detector_id));
diesel::joinable!(nodes -> process (locked_by));
diesel::joinable!(pending_measurements -> nodes (node_id));

diesel::allow_tables_to_appear_in_same_query!(
    detector,
    entangled_pair,
    links,
    measurements,
    nodes,
    pending_measurements,
    process,
);
