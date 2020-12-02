use std::time::SystemTime;

pub struct Organization {
    pub id: i64
}

pub struct Project {
    pub id: i64
}

pub struct User {
    pub id: i64
}

pub struct OrganizationMember {
    pub id: i64,
    pub organization_id: i64,
    pub user_id: i64
}

pub struct ProjectMember {
    pub id: i64,
    pub project_id: i64,
    pub user_id: i64
}

pub struct Role {
    pub id: i64,
    pub name: String,
}

pub struct ProjectMemberRole {
    pub id: i64,
    pub member_id: i64,
    pub role_id: i64,
    pub assigned_by_member_id: i64,
    pub assigned_unix_timestamp: i64
}

pub struct OrganizationMemberRole {
    pub id: i64,
    pub member_id: i64,
    pub role_id: i64,
    pub assigned_by_member_id: i64,
    pub assigned_unix_timestamp: i64
}
