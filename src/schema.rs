// @generated automatically by Diesel CLI.

diesel::table! {
    company (id) {
        id -> Int4,
        name -> Text,
        age -> Int4,
        address -> Nullable<Bpchar>,
        salary -> Nullable<Float4>,
    }
}

diesel::table! {
    pgbench_accounts (aid) {
        aid -> Int4,
        bid -> Nullable<Int4>,
        abalance -> Nullable<Int4>,
        filler -> Nullable<Bpchar>,
    }
}

diesel::table! {
    pgbench_branches (bid) {
        bid -> Int4,
        bbalance -> Nullable<Int4>,
        filler -> Nullable<Bpchar>,
    }
}

diesel::table! {
    pgbench_tellers (tid) {
        tid -> Int4,
        bid -> Nullable<Int4>,
        tbalance -> Nullable<Int4>,
        filler -> Nullable<Bpchar>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    company,
    pgbench_accounts,
    pgbench_branches,
    pgbench_tellers,
    posts,
);
