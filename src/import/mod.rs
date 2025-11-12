pub mod trust_import;
pub mod trust_site_import;

use sqlx::{Pool, Postgres};
use crate::AppError;
use std::path::PathBuf;


pub async fn import_data(data_folder: &PathBuf, pool: &Pool<Postgres>) -> Result<(), AppError> {

    let file_name = "etr.csv";
    trust_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ets.csv";
    trust_site_import::import_data(data_folder, file_name, pool).await?;

    Ok(())
}
