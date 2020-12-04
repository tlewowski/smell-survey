pub struct User {
    pub id: i64
}

pub struct Role {
    pub id: i64,
    pub name: String,
}

pub struct UserSystemRole {
    pub id: i64,
    pub role_id: i64,
    pub user_id: i64
}

pub struct Team {
    pub id: i64,
    pub parent_team_id: Option<i64>
}

pub struct TeamMember {
    pub id: i64,
    pub team_id: i64,
    pub user_id: i64
}

pub struct TeamMemberRole {
    pub id: i64,
    pub member_id: i64,
    pub role_id: i64
}

pub struct AuditTrailRecord {
    pub id: i64,
    pub unix_timestamp: i64,
    pub actor_id: i64,
    pub action_content_stringified: String,
    pub was_successful: bool
}
