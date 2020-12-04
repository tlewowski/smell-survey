table! {
    audit_trail (id) {
        id -> Int8,
        unix_timestamp -> Int8,
        actor_id -> Int8,
        action_content_stringified -> Text,
        was_successful -> Nullable<Bool>,
    }
}

table! {
    roles (id) {
        id -> Int8,
        name -> Text,
    }
}

table! {
    team_member_roles (id) {
        id -> Int8,
        member_id -> Int8,
        role_id -> Int8,
    }
}

table! {
    team_members (id) {
        id -> Int8,
        user_id -> Int8,
        team_id -> Int8,
    }
}

table! {
    teams (id) {
        id -> Int8,
        parent_team_id -> Nullable<Int8>,
    }
}

table! {
    user_system_roles (id) {
        id -> Int8,
        role_id -> Int8,
        user_id -> Int8,
    }
}

table! {
    users (id) {
        id -> Int8,
    }
}

joinable!(audit_trail -> users (actor_id));
joinable!(team_member_roles -> team_members (member_id));
joinable!(team_members -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    audit_trail,
    roles,
    team_member_roles,
    team_members,
    teams,
    user_system_roles,
    users,
);
