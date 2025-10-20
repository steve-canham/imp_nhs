//use crate::AppError;
use chrono::NaiveDate;
use regex::{Captures, Regex};

//use log::info;


pub fn convert_to_date(text: &str) -> Option<NaiveDate> {

    match NaiveDate::parse_from_str(text, "%Y%m%d")
        {
            Ok(d) => Some(d),
            Err(_) => None,
        }
}


pub fn capitalise_words(text: &str) -> String {
    
   let mut new_text = "".to_string();
   let lower = text.to_string().to_lowercase();
   
   let parts: Vec<_> = lower
      .split(|c: char| c == '-' || c.is_ascii_whitespace())
      .filter(|p| !p.is_empty())
      .collect();

   for w in parts {
        let mut c = w.chars();      // turn word into a vector of characters
        let mut wcap = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        };


        if wcap.len() == 2 {
            let short_word = wcap.to_uppercase();

            let short_word_slice = match short_word.as_str() {
                "ST" => "St",
                "ON" => "on",
                "OF" => "of",
                "DR" => "Dr",
                "AT" => "at",
                "NO" => "No",
                _ => &short_word
            };
            wcap = short_word_slice.to_string()
        }

        else if wcap.len() == 3 {

            let short_word = wcap.to_uppercase();
            let mut short_word_slice = short_word.as_str();

            if short_word.starts_with(['1', '2', '3', '4', '5', '6']) {
                short_word_slice = match short_word.as_str() {
                    "1ST" => "1st",
                    "2ND" => "2nd",
                    "3RD" => "3rd",
                    "4TH" => "4th",
                    "5TH" => "5th",
                    "6TH" => "6th",
                    _ => &short_word
                };
            }
            else if short_word.starts_with('A')
            {
                short_word_slice = match short_word.as_str() {
                    "AND" => "and",
                    "ALL" => "All",
                    "ASH" => "Ash",
                    "ARK" => "Ark",
                    "AMI" => "Ami",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['A', 'B', 'C', 'D']) {
                short_word_slice = match short_word.as_str() {
                    "BAR" => "Bar",
                    "BAY" => "Bay",

                    "COW" => "Cow",
                    "DAY" => "Day",
                    "DR." => "Dr",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['E', 'F']) {
                short_word_slice = match short_word.as_str() {
                    "ELM" => "Elm",
                    "EAR" => "Ear",
                    "EYE" => "Eye",
                    "END" => "End",
                    "FIR" => "Fir",
                    "FOR" => "for",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['G', 'H']) {
                short_word_slice = match short_word.as_str() {
                    "GEN" => "Gen",
                    "HEY" => "Hey",
                    "HUB" => "Hub",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['M', 'N']) {
                short_word_slice = match short_word.as_str() {
                    "MED" => "Med",
                    "MID" => "Mid",
                    "MON" => "Mon",
                    "NEW" => "New",
                    "NON" => "Non",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['O', 'P']) {
                short_word_slice = match short_word.as_str() {
                    "OFF" => "Off",
                    "OLD" => "Old",
                    "OAK" => "Oak",
                    "OUT" => "Out",

                    "PEN" => "Pen",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['R', 'S', 'T']) {
                 short_word_slice = match short_word.as_str() {
                    "RED" => "Red", 
                    "RAY" => "Ray",
                    "ROM" => "Rom",
                    "SPA" => "Spa",
                    "ST." => "St",
                    "THE" => "The",
                    "TOR" => "Tor",
                     _ => short_word_slice
                };
            }
            else {
                short_word_slice = match short_word.as_str() {
                    "WAY" => "Way",
                    "WYE" => "Wye",
                    "WAX" => "Wax",
                    "YEW" => "Yew", 
                    _ => &short_word_slice
                };
            }
            wcap = short_word_slice.to_string();
        }

        else if wcap.len() == 4 {
           
           let short_word_slice = match wcap.as_str() {
                "Crht" => "CRHT",
                "Adhd" => "ADHD",
                "Ftac" => "FTAC",
                "Cwmh" => "CWMH",
                "Nifs" => "NIFS",
                "Aecu" => "AECU",
                "Afrs" => "AFRS",
                "Cmht" => "CMHT",
                "Iapt" => "IAPT",
                "Cypd" => "CYPD",
                "Cyps" => "CYPS",
                "Daat" => "DAAT",
                "Ddtc" => "DDTC",
                "Fp10" => "FP10",
                "Hlht" => "HLHT",
                "Hmls" => "HMLS",
                "Mhlt" => "MHLT",
                "Rhch" => "RHCH",
                "Upon" => "upon",
                "Ymca" => "YMCA",
                _ => wcap.as_str()
            };

            wcap = short_word_slice.to_string()
        }

        else if wcap.len() == 5 {
           
           let short_word_slice = match wcap.as_str() {
                "Camhs" => "CAMHS",
                "Mhsop" => "MHSOP",
                "Crhtt" => "CRHTT",
                "Epact" => "ePact",
                "Ctaid" => "CTAID",
                "Daart" => "DAART",
                "Dairs" => "DAIRS",
                "Cofph" => "CPFPH",
                "Cowph" => "COWPH",               
                "Under" => "under",

                _ => wcap.as_str()
            };
            wcap = short_word_slice.to_string()
        }

        
        new_text = new_text + " " + &wcap;
   }

   new_text = new_text.replace("'","’");
   new_text = new_text.replace(".","");
   
   new_text.trim().to_string()
    
}


pub fn get_postal_address(a1: &str, a2: &str, a3: &str, city: &str, pcode: &str) -> (String, String) {

    let cap_city = capitalise_words(city);

    let a = capitalise_words(a1);
    let b = capitalise_words(a2);
    let c = capitalise_words(a3);

    let b = if b == "" {""} else {&(", ".to_string() + &b)};
    let c = if c == "" {""} else {&(", ".to_string() + &c)};
    let d = if cap_city == "" {""} else {&(", ".to_string() + &cap_city)};
    let e = if pcode == "" {""} else {&(", ".to_string() + pcode)};
    let postal_address = a + b + c + d + e;
    
    (cap_city, postal_address)
}

pub fn repair_brackets(text: &str) -> String {

    let mut new_text = text.to_string(); 
    
    // find position of left bracket - if no space before add one

    let re = Regex::new(r"\S\(").unwrap();
    if re.is_match(text)
    {
        new_text = new_text.replace("(", " (");
    }

    // if 4 or fewer letters in the brackets capitalise all
    // else capitalise the first letter of the rest

    let re = Regex::new(r"\((?<content>[^)]+)\)").unwrap();
    new_text = re.replace_all(&new_text, |caps: &Captures| {
       if caps["content"].len() < 5 
            {format!("({})", &caps["content"].to_uppercase())}
       else 
            {format!("({})", capitalise_words(&caps["content"]))}
    }).to_string();
    //new_text = re.replace_all(&new_text, "(FOO)" ).to_string();
    new_text
}


pub fn repair_site_name(text: &str) -> String {

    let new_text = text.to_string(); 
    let new_text = new_text.replace("(Epact", "(ePact");
    let new_text = new_text.replace("E Pact", "ePact");
    let new_text = new_text.replace("Cypmhs", "CYPMHS");
    let new_text = new_text.replace("Fp10hnc", "FP10HNC");
    new_text
}
  
