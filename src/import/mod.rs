pub mod rec_structs;

pub mod auth_import;
pub mod ccg_import;
pub mod ccg_site_import;
pub mod csu_import;
pub mod csu_site_import;



pub mod trust_import;
pub mod trust_site_import;


use sqlx::{Pool, Postgres};
use crate::AppError;
use std::path::PathBuf;


pub async fn import_data(data_folder: &PathBuf, pool: &Pool<Postgres>) -> Result<(), AppError> {

    /*

    execute_sql(get_care_trusts_sql(), pool).await?;
    execute_sql(get_care_trust_sites_sql(), pool).await?;
    execute_sql(get_hospices_sql(), pool).await?;
    execute_sql(get_iom_orgs_sql(), pool).await?;
    execute_sql(get_non_nhs_sql(), pool).await?;
    execute_sql(get_supp_agencies_sql(), pool).await?;
    execute_sql(get_exec_agencies_sql(), pool).await?;
    execute_sql(get_gpmem_sql(), pool).await?;
    execute_sql(get_pcns_sql(), pool).await?;
    execute_sql(get_pcn_partners_sql(), pool).await?;
    execute_sql(get_php_providers_sql(), pool).await?;
    execute_sql(get_php_provider_sites_sql(), pool).await?;
    execute_sql(get_gps_sql(), pool).await?;
    execute_sql(get_shas_sql(), pool).await?;
    execute_sql(get_trusts_sql(), pool).await?;
    execute_sql(get_treat_centres_sql(), pool).await?;
    execute_sql(get_trust_sites_sql(), pool).await?;
    execute_sql(get_ni_orgs_sql(), pool).await?;
    execute_sql(get_ni_gps_in_lhscg_sql(), pool).await?;
    execute_sql(get_ni_gps_sql(), pool).await?;
    execute_sql(get_successions_sql(), pool).await?;
    execute_sql(get_wlhbs_sql(), pool).await?;
    execute_sql(get_wlhb_sites_sql(), pool).await?;
    */

    let file_name = "eauth.csv";
    auth_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "eccg.csv";
    ccg_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "eccgsite.csv";
    ccg_site_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ecsu.csv";
    csu_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ecsusite.csv";
    csu_site_import::import_data(data_folder, file_name, pool).await?;



    let file_name = "etr.csv";
    trust_import::import_data(data_folder, file_name, pool).await?;

    let file_name = "ets.csv";
    trust_site_import::import_data(data_folder, file_name, pool).await?;

    Ok(())
}
