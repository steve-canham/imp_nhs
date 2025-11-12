use crate::AppError;
use crate::utils;
use crate::vectors::pcn_vectors::PCNVecs;
use super::rec_structs::PCNRec;

use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use log::info;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct PCNLine {
	pub ods_code: String,
	pub ods_name: String,
	pub subicb_loc: String,
	pub subicb_name: String,
    pub open_date: String,
	pub close_date: String,
	pub aline1: String,
	pub aline2: String,
	pub aline3: String,
	pub aline4: String,
	pub aline5: String,
	pub postcode: String,
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
    let mut dv: PCNVecs = PCNVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: PCNLine = result?;

        let mut site_name =  utils::capitalise_words(&source.ods_name);
        if site_name.contains('(') {
            site_name = utils::repair_brackets(&site_name)
        };
        site_name = utils::repair_site_name(&site_name);

        let (cap_city, postal_address) = utils::get_postal_address(&source.aline1, &source.aline2, 
                                                        &source.aline3, &source.aline4, &source.postcode);        
        
        let opened = utils::convert_to_date(&source.open_date);
        let closed = utils::convert_to_date(&source.close_date);
        
        let pcn_rec = PCNRec {
            ods_code: source.ods_code,
            ods_name: site_name,
            subicb_loc: source.subicb_loc,
            subicb_name: source.subicb_name,
            open_date: opened,
            close_date: closed,
            city: cap_city,
            postcode: source.postcode,
            postal_add: postal_address,

        };

        dv.add_data(&pcn_rec);   // transfer data to vectors
        i +=1;    

    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to ods.pcn", i, source_file_name);

    Ok(())
}