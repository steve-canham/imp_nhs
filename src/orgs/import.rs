use sqlx::{Pool, Postgres};
use crate::AppError;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use chrono::NaiveDate;

use super::data_vectors::OrgVecs;
use log::info;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct OrgLine {
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
    pub column17 : String,
    pub contact_tel: String,
    pub column19: String,
    pub column20: String,
    pub column21: String,
    pub amended_record: String,
    pub column23: String,
    pub column25: String,
    pub column26: String,
    pub column27: String,
 }

#[derive(Debug)]
pub struct OrgRec {
    pub ods_code : String,
    pub ods_name: String,
    pub grouping: String,
    pub health_geog : String,
    pub postal_add: String,
    pub city: String,
    pub open_date: Option<NaiveDate>,
    pub close_date: Option<NaiveDate>, 
}


pub async fn import_org_data(data_folder: &PathBuf, source_file_name: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {

    let source_file_path: PathBuf = [data_folder, &PathBuf::from(source_file_name)].iter().collect();
    let file = File::open(source_file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .quote(b'"')
        .from_reader(buf_reader);

    let mut i = 0;
    let vector_size = 500;
    let mut dv: OrgVecs = OrgVecs::new(vector_size);
            
    for result in csv_rdr.deserialize() {
    
        let source: OrgLine = result?;

        let cap_name =  capitalise_words(&source.ods_name);
        let cap_city = capitalise_words(&source.aline4);

        let a1 =  capitalise_words(&source.aline1);
        let a2 =  capitalise_words(&source.aline2);
        let a3 =  capitalise_words(&source.aline3);


        let b = if a2 == "" {""} else {&(", ".to_string() + &a2)};
        let c = if a3 == "" {""} else {&(", ".to_string() + &a3)};
        let d = if cap_city == "" {""} else {&(", ".to_string() + & cap_city)};
        let e = if source.postcode == "" {""} else {&(", ".to_string() + &source.postcode)};
        let postal_add = a1 + b + c + d + e;

        let opened = match NaiveDate::parse_from_str(&source.open_date, "%Y%m%d")
        {
            Ok(d) => Some(d),
            Err(_) => None,
        };

        let closed = match NaiveDate::parse_from_str(&source.close_date, "%Y%m%d")
        {
            Ok(d) => Some(d),
            Err(_) => None,
        };


        let org_rec = OrgRec {
            ods_code: source.ods_code,
            ods_name: cap_name,
            grouping: source.grouping,
            health_geog: source.health_geog,
            postal_add: postal_add,
            city: cap_city,
            open_date: opened,
            close_date: closed,
        };

        dv.add_data(&org_rec);   // transfer data to vectors
        i +=1;    

    }
            
    dv.store_data(&pool).await?;
    info!("{} records processed from {} to src.orgs", i, source_file_name);

    Ok(())
}


fn capitalise_words(text: &str) -> String {
    
   let mut new_text = "".to_string();
   let lower = text.to_string().to_lowercase();
   
   let parts: Vec<_> = lower
      .split(|c: char| c == '-' || c.is_ascii_whitespace())
      .filter(|p| !p.is_empty())
      .collect();

   for w in parts {
        let mut c = w.chars();      // turn word into a vector of characters
        let wcap = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        };
        
        let new_word = match wcap.as_str() {
            "Nhs" => "NHS",
            "And" => "and",
            "Of" => "of",
            "For" => "for",
            "On" => "on",
            "Under" => "under",
            "Upon" => "upon",
            "Hq" => "HQ",
            _ => &wcap
        };
        new_text = new_text + " " + new_word;
   }

   new_text = new_text.replace("'","’");
   new_text = new_text.replace(".","");
   
   new_text
    
}

