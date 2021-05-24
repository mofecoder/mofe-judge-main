table! {
    ar_internal_metadata (key) {
        key -> Varchar,
        value -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    clarifications (id) {
        id -> Bigint,
        contest_id -> Bigint,
        problem_id -> Nullable<Bigint>,
        user_id -> Bigint,
        question -> Varchar,
        answer -> Nullable<Varchar>,
        publish -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    contests (id) {
        id -> Bigint,
        slug -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        kind -> Varchar,
        penalty_time -> Integer,
        start_at -> Nullable<Datetime>,
        end_at -> Nullable<Datetime>,
        editorial_url -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    posts (id) {
        id -> Bigint,
        title -> Varchar,
        content -> Text,
        public_status -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    problems (id) {
        id -> Bigint,
        slug -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        contest_id -> Nullable<Bigint>,
        writer_user_id -> Bigint,
        position -> Nullable<Varchar>,
        uuid -> Nullable<Varchar>,
        difficulty -> Varchar,
        execution_time_limit -> Integer,
        statement -> Varchar,
        constraints -> Varchar,
        input_format -> Varchar,
        output_format -> Varchar,
        checker_path -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    registrations (id) {
        id -> Bigint,
        user_id -> Bigint,
        contest_id -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    schema_migrations (version) {
        version -> Varchar,
    }
}

table! {
    submits (id) {
        id -> Bigint,
        user_id -> Integer,
        problem_id -> Bigint,
        path -> Varchar,
        status -> Varchar,
        point -> Nullable<Integer>,
        execution_time -> Nullable<Integer>,
        execution_memory -> Nullable<Integer>,
        compile_error -> Nullable<Text>,
        lang -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    testcases (id) {
        id -> Bigint,
        problem_id -> Bigint,
        name -> Nullable<Varchar>,
        input -> Nullable<Longtext>,
        output -> Nullable<Longtext>,
        explanation -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    testcase_results (id) {
        id -> Bigint,
        submit_id -> Bigint,
        testcase_id -> Bigint,
        status -> Varchar,
        execution_time -> Integer,
        execution_memory -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    testcase_sets (id) {
        id -> Bigint,
        problem_id -> Bigint,
        name -> Varchar,
        points -> Integer,
        is_sample -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    testcase_testcase_sets (id) {
        id -> Bigint,
        testcase_id -> Bigint,
        testcase_set_id -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    tester_relations (id) {
        id -> Bigint,
        problem_id -> Bigint,
        tester_user_id -> Bigint,
        approved -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    users (id) {
        id -> Bigint,
        provider -> Varchar,
        uid -> Varchar,
        encrypted_password -> Varchar,
        reset_password_token -> Nullable<Varchar>,
        reset_password_sent_at -> Nullable<Datetime>,
        allow_password_change -> Nullable<Bool>,
        remember_created_at -> Nullable<Datetime>,
        sign_in_count -> Integer,
        current_sign_in_at -> Nullable<Datetime>,
        last_sign_in_at -> Nullable<Datetime>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmation_token -> Nullable<Varchar>,
        confirmed_at -> Nullable<Datetime>,
        confirmation_sent_at -> Nullable<Datetime>,
        unconfirmed_email -> Nullable<Varchar>,
        role -> Varchar,
        name -> Nullable<Varchar>,
        atcoder_id -> Nullable<Varchar>,
        atcoder_rating -> Nullable<Integer>,
        writer_request_code -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        tokens -> Nullable<Text>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
    }
}

joinable!(problems -> contests (contest_id));
joinable!(problems -> users (writer_user_id));
joinable!(submits -> problems (problem_id));
joinable!(testcase_sets -> problems (problem_id));
joinable!(testcase_testcase_sets -> testcase_sets (testcase_set_id));
joinable!(testcase_testcase_sets -> testcases (testcase_id));
joinable!(testcases -> problems (problem_id));
joinable!(tester_relations -> problems (problem_id));
joinable!(tester_relations -> users (tester_user_id));

allow_tables_to_appear_in_same_query!(
    ar_internal_metadata,
    clarifications,
    contests,
    posts,
    problems,
    registrations,
    schema_migrations,
    submits,
    testcases,
    testcase_results,
    testcase_sets,
    testcase_testcase_sets,
    tester_relations,
    users,
);
