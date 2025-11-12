
pub mod setup;
pub mod err;
mod import;
mod vectors;
mod utils;


use setup::cli_reader;
use err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs;

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    let cli_pars: cli_reader::CliPars;
    cli_pars = cli_reader::fetch_valid_arguments(args)?;
    let flags = cli_pars.flags;

    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                                .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
                              
    let params = setup::get_params(cli_pars, &config_string)?;
    setup::establish_log(&params)?;
    let pool = setup::get_db_pool().await?;
         
    if flags.import_data   // normally should be true
    {
        // recreate the tables

        setup::create_tables(&pool).await?;

        // Import the data
       
        import::import_data(&params.data_folder, &pool).await?;
       
     }

     Ok(())  
}
