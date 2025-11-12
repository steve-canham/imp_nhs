use chrono::NaiveDate;

#[derive(Debug)]
pub struct AuthRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct CCGRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct CCGSiteRec {
	pub ods_code : String,
	pub ods_name: String,
	pub grouping: String,
	pub health_geog : String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub parent_org: String,
	pub join_parent_date: Option<NaiveDate>,
	pub left_parent_date: Option<NaiveDate>, 
}

#[derive(Debug)]
pub struct CSURec {
	pub ods_code : String,
	pub ods_name: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub subtype_code: String,
}

#[derive(Debug)]
pub struct CSUSiteRec {
	pub ods_code : String,
	pub ods_name: String,
	pub city: String,
	pub postcode: String,
	pub postal_add: String,
	pub open_date: Option<NaiveDate>,
	pub close_date: Option<NaiveDate>, 
	pub parent_org: String,
	pub join_parent_date: Option<NaiveDate>,
	pub left_parent_date: Option<NaiveDate>, 
}



#[derive(Debug)]
pub struct TrustRec {
    pub ods_code : String,
    pub ods_name: String,
    pub grouping: String,
    pub health_geog : String,
    pub city: String,
    pub postcode: String,
    pub postal_add: String,
    pub open_date: Option<NaiveDate>,
    pub close_date: Option<NaiveDate>, 
}



#[derive(Debug)]
pub struct TrustSiteRec {
    pub ods_code : String,
    pub ods_name: String,
    pub grouping: String,
    pub health_geog : String,
    pub city: String,
    pub postcode: String,
    pub postal_add: String,
    pub open_date: Option<NaiveDate>,
    pub close_date: Option<NaiveDate>, 
    pub subtype_code: String,
    pub parent_org: String,
}