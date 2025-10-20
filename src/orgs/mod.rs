mod data_vectors;
mod import;

use sqlx::{Pool, Postgres};
use crate::AppError;
use std::path::PathBuf;
//use log::info;

pub async fn create_org_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists src.orgs;
                create table src.orgs (
                  ods_code         varchar    not null primary key
                , ods_name         varchar    not null
                , grouping         varchar    null
                , health_geog      varchar    null
                , postal_add       varchar    null
                , city             varchar    null
                , open_date        varchar    null
                , close_date       varchar    null
            );"#;
            
    sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn import_data(data_folder: &PathBuf, source_file_name: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {

    import::import_org_data(data_folder, source_file_name, pool).await?;

    //transfer_orgs(pool).await?;
    //process_org_names(pool).await?;

    Ok(())
}

/*
async fn transfer_orgs(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into src.countries (id, rank, iso_code, country_name,
              continent, tld, capital)
              select id, rank, iso_code, country_name,
              continent, tld, capital
              from geo.countries
              order by country_name;"#;

    let res = sqlx::raw_sql(sql).execute(pool)
                .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    info!("{} country records transferred to geo schema", res.rows_affected());

    Ok(())
}


async fn process_org_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into src.country_names (country_id, country_name, alt_name, langlist)
        select g.id, g.country_name, a.alt_name, a.langs
        from src.countries g
        inner join geo.alt_names a
        on g.id = a.id;"#;

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    info!("{} country name records created", res.rows_affected());

    Ok(())
}
 */
