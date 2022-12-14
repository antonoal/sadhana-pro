// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "practice_data_type_enum"))]
    pub struct PracticeDataTypeEnum;
}

diesel::table! {
    diary (cob_date, user_id, practice_id) {
        cob_date -> Date,
        user_id -> Uuid,
        practice_id -> Uuid,
        value -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PracticeDataTypeEnum;

    user_practices (id) {
        id -> Uuid,
        user_id -> Uuid,
        practice -> Text,
        data_type -> PracticeDataTypeEnum,
        valid_from -> Nullable<Date>,
        valid_to -> Nullable<Date>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        hash -> Text,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(diary -> user_practices (practice_id));
diesel::joinable!(diary -> users (user_id));
diesel::joinable!(user_practices -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    diary,
    user_practices,
    users,
);
