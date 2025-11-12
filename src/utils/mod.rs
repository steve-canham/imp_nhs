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
                "AT" => "at",
                "DR" => "Dr",
                "DU" => "Du",   
                "IN" => "in",
                "NO" => "No",             
                "OF" => "of",
                "ON" => "on",
                "ST" => "St",
                "TY" => "Ty",
                "YR" => "Yr",
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
                    "ANN" => "Ann",
                    "ALL" => "All",
                    "ASH" => "Ash",
                    "ARK" => "Ark",
                    "AMI" => "Ami",
                    "AMY" => "Amy",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['B', 'C', 'D']) {
                short_word_slice = match short_word.as_str() {
                    "BAR" => "Bar",
                    "BAY" => "Bay",
                    "BOW" => "Bow",
                    "CTR" => "Ctr",
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
                    "ESK" => "Esk",
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
                    "HAM" => "Ham",
                    "HEY" => "Hey",
                    "HOB" => "Hob",
                    "HUB" => "Hub",
                    "HPL" => "Hpl",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['L', 'M', 'N']) {
                short_word_slice = match short_word.as_str() {
                    "LOW" => "Low",
                    "MED" => "Med",
                    "MID" => "Mid",
                    "MON" => "Mon",
                    "NEW" => "New",
                    "NON" => "Non",
                    "NUR" => "Nur",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['O', 'P']) {
                short_word_slice = match short_word.as_str() {
                    "OFF" => "Off",
                    "OLD" => "Old",
                    "OAK" => "Oak",
                    "OUR" => "Our",
                    "OUT" => "Out",
                    "ONE" => "One",
                    "OWN" => "Own",
                    "PEN" => "Pen",
                    "PRE" => "Pre",
                    _ => short_word_slice
                };
            }
            else if short_word.starts_with(['R', 'S', 'T']) {
                 short_word_slice = match short_word.as_str() {
                    "RED" => "Red", 
                    "RAY" => "Ray",
                    "ROY" => "Roy",
                    "ROM" => "Rom",
                    "SEA" => "Sea",
                    "SIX" => "Six",
                    "SPA" => "Spa",
                    "ST." => "St",
                    "SUB" => "Sub",
                    "THE" => "The",
                    "TOR" => "Tor",
                     _ => short_word_slice
                };
            }
            else if short_word.starts_with(['W', 'Y']) {
                short_word_slice = match short_word.as_str() {
                    "WAR" => "War",
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
          
            if wcap.starts_with('A') {
                wcap = match wcap.as_str() {
                    "Adhd" => "ADHD".to_string(),
                    "Aecu" => "AECU".to_string(),
                    "Afrs" => "AFRS".to_string(),
                    _ => wcap
                };
            }
            else if wcap.starts_with('C')
            {
                wcap = match wcap.as_str() {
                    "Camh" => "CAMH".to_string(),
                    "Cdat" => "CDAT".to_string(),
                    "Crht" => "CRHT".to_string(),
                    "Chbt" => "CHBT".to_string(),
                    "Cldt" => "CLDT".to_string(),
                    "Cmht" => "CMHT".to_string(),
                    "Cwmh" => "CWMH".to_string(),
                    "Cypd" => "CYPD".to_string(),
                    "Cyps" => "CYPS".to_string(),
                    _ => wcap
                };
            }
            else if wcap.starts_with(['D', 'F'])
            {
                wcap = match wcap.as_str() {
                    "Daat" => "DAAT".to_string(),
                    "Ddtc" => "DDTC".to_string(),
                    "Fp10" => "FP10".to_string(),
                    "Ftac" => "FTAC".to_string(),
                    _ => wcap
                };
            }
            else if wcap.starts_with(['G', 'H', 'I'])
            {
                wcap = match wcap.as_str() {
                    "Gstt" => "GSTT".to_string(),
                    "Hbss" => "HBSS".to_string(),
                    "Hlht" => "HLHT".to_string(),
                    "Hmls" => "HMLS".to_string(),
                    "Iapt" => "IAPT".to_string(),
                    _ => wcap
                };
            }
            else if wcap.starts_with(['M', 'N'])
            {
                wcap = match wcap.as_str() {
                    "Mhlt" => "MHLT".to_string(),
                    "Nifs" => "NIFS".to_string(),
                    "Nihr" => "NIHR".to_string(),
                    "Nlfs" => "NLFS".to_string(),
                    _ => wcap
                };
            }
            else if wcap.starts_with(['R', 'U', 'Y'])
            {
                wcap = match wcap.as_str() {
                    "Rhch" => "RHCH".to_string(),
                    "Uclh" => "UCLH".to_string(),
                    "Upon" => "upon".to_string(),
                    "Ymca" => "YMCA".to_string(),
                    _ => wcap
                };
            }
        }

        else if wcap.len() == 5 {

            if wcap.starts_with(['C']) {
                wcap = match wcap.as_str() {
                    "Camhs" => "CAMHS".to_string(),
                    "Cofph" => "CPFPH".to_string(),
                    "Cowph" => "COWPH".to_string(),  
                    "Crhtt" => "CRHTT".to_string(),
                    "Ctaid" => "CTAID".to_string(),
                    "Ctpld" => "CTPLD".to_string(),
                    _ => wcap
                };
            }
            else if wcap.starts_with('D')
            {
                wcap = match wcap.as_str() {
                    "Daart" => "DAART".to_string(),
                    "Dscro" => "DSCRO".to_string(),
                    "Dairs" => "DAIRS".to_string(),
                    _ => wcap
                };
            }
            else if wcap.starts_with(['E', 'I', 'M'])
            {
                wcap = match wcap.as_str() {
                    "Epact" => "ePact".to_string(),
                    "Idass" => "IDASS".to_string(),
                    "Mhcas" => "MHCAS".to_string(),
                    "Mhsop" => "MHSOP".to_string(),
                    _ => wcap
                };
            }
            else if wcap.starts_with(['S', 'L', 'U'])
            {
                wcap = match wcap.as_str() {
                    "Spfit" => "SPFIT".to_string(),
                    "Lpfit" => "LPFIT".to_string(),
                    "Under" => "under".to_string(),
                    _ => wcap
                };
            }
        }

        else if wcap.len() == 7 {
           
           let short_word_slice = match wcap.as_str() {
                "Nmepfit" => "NMEPFIT",
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
    new_text
}


pub fn repair_site_name(text: &str) -> String {

    let new_text = text.to_string(); 
    let new_text = new_text.replace("(Epact", "(ePact");
    let new_text = new_text.replace("E Pact", "ePact");
    let new_text = new_text.replace("Cypmhs", "CYPMHS");
    let new_text = new_text.replace("Fp10hnc", "FP10HNC");
    let new_text = new_text.replace("Y AMH ", "YAMH ");
    new_text
}
  
