use super::import::OrgRec;
use crate::AppError;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use chrono::NaiveDate;

pub struct OrgVecs {
    pub codes: Vec<String>,
    pub names: Vec<String>,
    pub groupings: Vec<String>,
    pub health_geogs: Vec<String>,
    pub postal_adds: Vec<String>,
    pub cities: Vec<String>,
    pub open_dates: Vec<Option<NaiveDate>>,
    pub close_dates: Vec<Option<NaiveDate>>,
}


impl OrgVecs{
    pub fn new(vsize: usize) -> Self {
        OrgVecs { 
            codes: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            groupings: Vec::with_capacity(vsize),
            health_geogs: Vec::with_capacity(vsize),
            postal_adds: Vec::with_capacity(vsize),
            cities: Vec::with_capacity(vsize),
            open_dates: Vec::with_capacity(vsize),
            close_dates: Vec::with_capacity(vsize),
        }
    }

    pub fn add_data(&mut self, r: &OrgRec) 
    {
        self.codes.push(r.ods_code.clone());
        self.names.push(r.ods_name.clone());
        self.groupings.push(r.grouping.clone());
        self.health_geogs.push(r.health_geog.clone());
        self.postal_adds.push(r.postal_add.clone());
        self.cities.push(r.city.clone());
        self.open_dates.push(r.open_date.clone());
        self.close_dates.push(r.close_date.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO src.orgs (ods_code, ods_name, grouping, health_geog, 
                      postal_add, city, open_date, close_date) 
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[], $4::text[], $5::text[], 
                $6::text[], $7::text[], $8::text[]);"#;

        sqlx::query(&sql)
        .bind(&self.codes).bind(&self.names).bind(&self.groupings).bind(&self.health_geogs)
        .bind(&self.postal_adds).bind(&self.cities).bind(&self.open_dates).bind(&self.close_dates)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}
