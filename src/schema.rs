// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Int4,
        date -> Date,
        description -> Nullable<Varchar>,
        category -> Varchar,
        amount -> Float8,
    }
}
