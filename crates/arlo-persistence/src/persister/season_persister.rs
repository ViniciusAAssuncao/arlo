use crate::error::PersistenceResult;
use crate::models::season::{
    FixtureRow, KnockoutTieRow, PostponementRecordRow, SeasonInstanceRow, SeasonStageRow,
};
use crate::repositories;
use sqlx::SqlitePool;

pub struct SeasonPersister;

impl SeasonPersister {
    pub async fn persist_season_schedule(
        pool: &SqlitePool,
        season_instance: &SeasonInstanceRow,
        stages: &[SeasonStageRow],
        fixtures: &[FixtureRow],
    ) -> PersistenceResult<()> {
        let mut tx = pool.begin().await?;

        repositories::season::season_instances::insert(&mut tx, season_instance).await?;
        repositories::season::season_stages::insert_batch(&mut tx, stages).await?;
        repositories::season::fixtures::insert_batch(&mut tx, fixtures).await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn persist_stage_schedule(
        pool: &SqlitePool,
        stage: &SeasonStageRow,
        fixtures: &[FixtureRow],
        ties: &[KnockoutTieRow],
    ) -> PersistenceResult<()> {
        let mut tx = pool.begin().await?;

        repositories::season::season_stages::insert(&mut tx, stage).await?;
        repositories::season::fixtures::insert_batch(&mut tx, fixtures).await?;
        if !ties.is_empty() {
            repositories::season::knockout_ties::insert_batch(&mut tx, ties).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn persist_postponements(
        pool: &SqlitePool,
        updated_fixtures: &[FixtureRow],
        postponements: &[PostponementRecordRow],
    ) -> PersistenceResult<()> {
        let mut tx = pool.begin().await?;

        repositories::season::fixtures::update_batch(&mut tx, updated_fixtures).await?;
        repositories::season::postponements::insert_batch(&mut tx, postponements).await?;

        tx.commit().await?;

        Ok(())
    }
}