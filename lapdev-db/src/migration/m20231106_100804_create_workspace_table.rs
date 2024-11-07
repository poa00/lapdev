use sea_orm_migration::prelude::*;

use crate::migration::{
    m20231105_152940_create_machine_type_table::MachineType,
    m20231105_193627_create_workspace_host_table::WorkspaceHost,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Workspace::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Workspace::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Workspace::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Workspace::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Workspace::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Workspace::UserId).uuid().not_null())
                    .col(ColumnDef::new(Workspace::ProjectId).uuid())
                    .col(ColumnDef::new(Workspace::PrebuildId).uuid())
                    .col(ColumnDef::new(Workspace::Name).string().not_null())
                    .col(ColumnDef::new(Workspace::Service).string())
                    .col(ColumnDef::new(Workspace::Status).string().not_null())
                    .col(ColumnDef::new(Workspace::RepoUrl).string().not_null())
                    .col(ColumnDef::new(Workspace::RepoName).string().not_null())
                    .col(ColumnDef::new(Workspace::Branch).string().not_null())
                    .col(ColumnDef::new(Workspace::Commit).string().not_null())
                    .col(ColumnDef::new(Workspace::HostId).uuid().not_null())
                    .col(ColumnDef::new(Workspace::Osuser).string().not_null())
                    .col(ColumnDef::new(Workspace::SshPort).integer())
                    .col(ColumnDef::new(Workspace::IdePort).integer())
                    .col(ColumnDef::new(Workspace::SshPrivateKey).string().not_null())
                    .col(ColumnDef::new(Workspace::SshPublicKey).string().not_null())
                    .col(ColumnDef::new(Workspace::Env).string())
                    .col(ColumnDef::new(Workspace::Cores).string().not_null())
                    .col(ColumnDef::new(Workspace::UsageId).integer())
                    .col(ColumnDef::new(Workspace::MachineTypeId).uuid().not_null())
                    .col(ColumnDef::new(Workspace::BuildOutput).string())
                    .col(ColumnDef::new(Workspace::LastInactivity).timestamp_with_time_zone())
                    .col(ColumnDef::new(Workspace::AutoStart).boolean().not_null())
                    .col(ColumnDef::new(Workspace::AutoStop).integer())
                    .col(ColumnDef::new(Workspace::IsCompose).boolean().not_null())
                    .col(ColumnDef::new(Workspace::ComposeParent).uuid())
                    .col(ColumnDef::new(Workspace::Pinned).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Workspace::Table)
                            .from_col(Workspace::MachineTypeId)
                            .to_tbl(MachineType::Table)
                            .to_col(MachineType::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Workspace::Table)
                            .from_col(Workspace::HostId)
                            .to_tbl(WorkspaceHost::Table)
                            .to_col(WorkspaceHost::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("workspace_name_deleted_at_idx")
                    .table(Workspace::Table)
                    .unique()
                    .nulls_not_distinct()
                    .col(Workspace::Name)
                    .col(Workspace::DeletedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("workspace_organization_id_user_id_deleted_at_idx")
                    .table(Workspace::Table)
                    .col(Workspace::OrganizationId)
                    .col(Workspace::UserId)
                    .col(Workspace::DeletedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("workspace_host_id_deleted_at_status_idx")
                    .table(Workspace::Table)
                    .col(Workspace::HostId)
                    .col(Workspace::DeletedAt)
                    .col(Workspace::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("workspace_prebuild_id_deleted_at_idx")
                    .table(Workspace::Table)
                    .col(Workspace::PrebuildId)
                    .col(Workspace::DeletedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("workspace_compose_parent_deleted_at_idx")
                    .table(Workspace::Table)
                    .col(Workspace::ComposeParent)
                    .col(Workspace::DeletedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Workspace {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    OrganizationId,
    UserId,
    ProjectId,
    PrebuildId,
    Name,
    Service,
    Status,
    RepoUrl,
    RepoName,
    Branch,
    Commit,
    HostId,
    Osuser,
    SshPort,
    IdePort,
    SshPrivateKey,
    SshPublicKey,
    Env,
    Cores,
    MachineTypeId,
    BuildOutput,
    UsageId,
    AutoStart,
    AutoStop,
    LastInactivity,
    IsCompose,
    ComposeParent,
    Pinned,
}
