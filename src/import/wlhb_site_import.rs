use crate::AppError;
use crate::utils;
use crate::vectors::wlhb_site_vectors::WSiteVecs;
use super::rec_structs::WSiteRec;

use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use log::info;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct WSiteLine {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub aline1: String,
	pub aline2: String,
	pub aline3: String,
	pub aline4: String,
	pub aline5: String,
	pub postcode: String,
	pub open_date: String,
	pub close_date: String,
    pub column13: String,
	pub column14: String,
	pub column15: String,
	pub column16: String,
	pub column17: String,
	pub contact_tel: String,
	pub column19: String,
	pub column20: String,
	pub column21: String,
	pub amended_record: String,
	pub column23: String,
	pub column24: String,
	pub column25: String,
	pub column26: String,
	pub column27: String,
}


pub async fn import_data(data_folder: &PathBuf, source_file_name: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {

    let source_file_path: PathBuf = [data_folder, &PathBuf::from(source_file_name)].iter().collect();
    let file = File::open(source_file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .quote(b'"')
        .from_reader(buf_reader);
    
    let mut i = 0;
    let vector_size = 10000;
    let mut dv: WSiteVecs = WSiteVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: WSiteLine = result?;

        let site_name =  utils::capitalise_site_name(&source.ods_name);
        let (cap_city, postal_address) = utils::get_postal_address(&source.aline1, &source.aline2, 
                                                        &source.aline3, &source.aline4, &source.postcode);        
        
        let opened = utils::convert_to_date(&source.open_date);
        let closed = utils::convert_to_date(&source.close_date);
        
        let wlhb_site_rec = WSiteRec {
            ods_code: source.ods_code,
            ods_name: site_name,
            grouping: source.grouping,
            health_geog: source.health_geog,
            city: cap_city,
            postcode: source.postcode,
            postal_add: postal_address,
            open_date: opened,
            close_date: closed,
        };

        dv.add_data(&wlhb_site_rec);   // transfer data to vectors
        i +=1;    

    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.wlhb_sites", i, source_file_name);

    Ok(())
}