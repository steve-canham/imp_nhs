mod data_vectors;
mod import;

use sqlx::{Pool, Postgres};
use crate::AppError;
use std::path::PathBuf;
//use log::info;

pub async fn create_site_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists src.sites;
                create table src.sites (
                  ods_code         varchar    not null primary key
                , ods_name         varchar    not null
                , grouping         varchar    null
                , health_geog      varchar    null
                , city             varchar    null
                , postcode         varchar    null
                , postal_add       varchar    null
                , open_date        varchar    null
                , close_date       varchar    null
                , subtype_code     varchar    null
                , parent_org       varchar    null
             );"#;
            
    sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;


    Ok(())
}


pub async fn import_data(data_folder: &PathBuf, source_file_name: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {

    import::import_site_data(data_folder, source_file_name, pool).await?;
    //transfer_sites_data(pool).await?;
    //process_site_names(pool).await?;
    //create_distinct_sites(pool).await?;
    //create_org_site_links(pool).await?;

    Ok(())
}

/*
async fn transfer_sites_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"insert into src.cities(id, name, disamb_type, disamb_id, disamb_code, disamb_name, 
    country_id, country_code, country_name, lat, lng, pop)
    select id, name, disamb_type, disamb_id, disamb_code, disamb_name, 
    country_id, country_code, country_name, lat, lng, pop
    from geo.cities"#;

    let res = sqlx::raw_sql(sql).execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    info!("{} city records transferred to geo schema", res.rows_affected());

    Ok(())
}


async fn process_site_names(pool: &Pool<Postgres>) -> Result<(), AppError> {

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


async fn create_distinct_sites(pool: &Pool<Postgres>) -> Result<(), AppError> {
    
    let sql = r#"insert into src.city_names (city_id, city_name, disamb_id, 
         disamb_name, country_id, country_name, alt_name, langlist)
         select c.id, c.name, c.disamb_id, c.disamb_name, c.country_id, 
         c.country_name, a.alt_name, a.langs
         from src.cities c 
         inner join geo.alt_names a
         on c.id = a.id;"#;

    let res = sqlx::raw_sql(sql).execute(pool)
    .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    info!("{} city name records created", res.rows_affected());

    Ok(())
}


async fn create_org_site_links(pool: &Pool<Postgres>) -> Result<(), AppError> {
    
    // It appears that in the geonames data about 4600 cities (out of a total of 143,000+)
    // do not have an alt_name matching the geoname city_name - the folowing ensures that all
    // names are in the city_names table

    let sql = r#"SET client_min_messages TO WARNING; 
         drop table if exists src.temp_city_match;

         create table src.temp_city_match as 
         select *
         from src.city_names
         where city_name = alt_name;"#;

    sqlx::raw_sql(sql).execute(pool)
         .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
     
    let sql = r#"insert into src.city_names
         (city_id, city_name, disamb_id, disamb_name, 
         country_id, country_name, alt_name)
         select distinct n.city_id, n.city_name, 
         n.disamb_id, n.disamb_name, n.country_id, 
         n.country_name, n.city_name as alt_name
         from src.city_names n
         left join src.temp_city_match m
         on n.city_id = m.city_id
         where m.id is null;
         
         drop table src.temp_city_match;"#;

    let res = sqlx::raw_sql(sql).execute(pool)
    .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    info!("{} missing city default names added to the city_names table", res.rows_affected());

    Ok(())
}
 */




