use super::import::SiteRec;
use crate::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;

pub struct SiteVecs {
    pub codes: Vec<String>,
    pub names: Vec<String>,
    pub groupings: Vec<String>,
    pub health_geogs: Vec<String>,
    pub aline1s: Vec<String>,
    pub aline2s: Vec<String>,
    pub aline3s: Vec<String>,
    pub cities: Vec<String>,
    pub postcodes: Vec<String>,
    pub open_dates: Vec<Option<NaiveDate>>,
    pub close_dates: Vec<Option<NaiveDate>>,
    pub subtype_codes: Vec<String>,
    pub parent_orgs: Vec<String>,
}


impl SiteVecs{
    pub fn new(vsize: usize) -> Self {
        SiteVecs {
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            groupings: Vec::with_capacity(vsize),
            health_geogs: Vec::with_capacity(vsize),
            aline1s: Vec::with_capacity(vsize),
            aline2s: Vec::with_capacity(vsize),
            aline3s: Vec::with_capacity(vsize),
            cities: Vec::with_capacity(vsize),
            postcodes: Vec::with_capacity(vsize),
            open_dates: Vec::with_capacity(vsize),
            close_dates: Vec::with_capacity(vsize),
            subtype_codes: Vec::with_capacity(vsize),
            parent_orgs: Vec::with_capacity(vsize),
        }
    }


    pub fn add_data(&mut self, r: &SiteRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.names.push(r.ods_name.clone());
        self.groupings.push(r.grouping.clone());
        self.health_geogs.push(r.health_geog.clone());
        self.aline1s.push(r.aline1.clone());
        self.aline2s.push(r.aline2.clone());
        self.aline3s.push(r.aline3.clone());
        self.cities.push(r.city.clone());
        self.postcodes.push(r.postcode.clone());
        self.open_dates.push(r.open_date.clone());
        self.close_dates.push(r.close_date.clone());
        self.subtype_codes.push(r.subtype_code.clone());
        self.parent_orgs.push(r.parent_org.clone());
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO src.sites (ods_code, ods_name, grouping, health_geog, 
                      aline1, aline2, aline3, city, postcode, open_date, close_date, subtype_code, parent_org) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                $6::text[], $7::text[], $8::text[], $9::text[], $10::date[], $11::date[], $12::text[], $13::text[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names).bind(&self.groupings).bind(&self.health_geogs)
        .bind(&self.aline1s).bind(&self.aline2s).bind(&self.aline3s).bind(&self.cities)
        .bind(&self.postcodes).bind(&self.open_dates).bind(&self.close_dates).bind(&self.subtype_codes).bind(&self.parent_orgs)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}
