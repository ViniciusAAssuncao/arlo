use crate::error::PersistenceResult;
use crate::models::season::PromotionRelegationResultRow;
use crate::repositories::season::promotion_relegation_repository;
use sqlx::SqlitePool;

pub struct PromotionRelegationResultPersister;

impl PromotionRelegationResultPersister {
    pub async fn persist_results(
        pool: &SqlitePool,
        results: &[PromotionRelegationResultRow],
    ) -> PersistenceResult<()> {
        if results.is_empty() {
            return Ok(());
        }

        let mut tx = pool.begin().await?;
        promotion_relegation_repository::insert_batch(&mut tx, results).await?;
        tx.commit().await?;

        Ok(())
    }
}
