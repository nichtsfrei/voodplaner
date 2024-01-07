use std::{collections::HashMap, error::Error, fmt::Display};

use voodplaner_core::*;

#[derive(Debug)]
enum AmountError {
    UnknownAmount(String),
}

impl Display for AmountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmountError::UnknownAmount(x) => write!(f, "{x} is an unknown amount identifier"),
        }
    }
}

impl Error for AmountError {}

fn parse_value(id: &str, value: &str) -> Result<Amount, Box<dyn Error>> {
    let value = value
        .replacen("max. ", "", 1)
        .replacen(",", ".", 1)
        .replacen("≈ ", "", 1)
        .replacen(">", "", 1);
    let value = value
        .splitn(2, "-")
        .next()
        .expect("expected value at index 3");
    let value = value
        .splitn(2, "/")
        .next()
        .expect("expected value at index 3");
    let value = value
        .splitn(2, " ")
        .next()
        .expect("expected value at index 3");
    Ok(match id {
        "% der Energie" => {
            let init: f32 = value.parse()?;
            let normalized: f32 = init / 100.0;
            Amount::PercentageOfEnergy(normalized)
        },
        "g/kg KG/Tag" => Amount::GrammPerDay(value.parse()?),
        "mg/Tag" => Amount::MilliGramPerDay(value.parse()?),
        "g/Tag" => Amount::GrammPerDay(value.parse()?),
        "µg-RAE/Tag" => Amount::MicroGramPerDay(value.parse()?),
        "µg/Tag" => Amount::MicroGramPerDay(value.parse()?),
        id => return Err(Box::new(AmountError::UnknownAmount(id.to_owned()))),
    })
}

// Nutrient recommandation
//
// The energy nutrient is ignored as we calculate it per person rather than using a lookup
// reference.
//
// It is identified by the index 2 of the csv file of
// https://www.dge.de/wissenschaft/referenzwerte-tool
//

const CSV: &'static str = include_str!("../dge-references.csv");
pub fn generate() -> Result<String, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(std::io::Cursor::new(CSV));
    let mut results = rdr.records();
    // ignore header definitions
    let _ = results.next();
    // TODO create lookup tables per start arge and gender rather than infants ...
    // this would make the loops much faster in the using library
    // the lookup key has to be [MALE|FEMALE]_\d+_[MONTHS|YEARS]
    let mut lookup: HashMap<(Sex, Group, String), Vec<Nutrient>> = HashMap::new();

    for result in results {
        let record = result?;
        // skip indicator
        match record.get(5) {
            Some("Für diese Altersgruppe liegt kein Referenzwert vor.")
            | Some("Säuglinge sollen keinen Alkohol trinken.")
            | Some("Kinder und Jugendliche sollen keinen Alkohol trinken.") => continue,
            _ => {}
        };
        let group = {
            let gl: Vec<&str> = record
                .get(0)
                .expect("expected group value")
                .split(" ")
                .collect();
            let start_age: u8 = gl[1].parse()?;
            let end_age: Option<u8> = gl[4].parse::<u8>().ok();
            match gl[0] {
                "Säuglinge" => Group::Infants(start_age, end_age.expect("end age on infants")),
                "Kinder" => Group::Children(start_age, end_age.expect("end age on children")),
                "Jugendliche" => Group::Teenager(start_age, end_age.expect("end age on teens")),
                "Erwachsene" | "Ewachsene" => Group::Adults(start_age, end_age),
                err => panic!("unexpected {}", err),
            }
        };
        // the original calls it Geschlecht, which literally translates to gender. However I think
        // the organs of producing the hormons are relevant. Since the alternative can only be done
        // via an hormon therapy this should just apply to adults or late teenagers and therefore
        // should be safe to choose freely between hence I used sex to not be in that fluent
        // space.
        let sex = {
            match record.get(1).expect("expectted sex value at index 1") {
                "Männlich" => Sex::Male,
                "Weiblich" => Sex::Female,
                x => panic!("unexpected {}", x),
            }
        };
        let nutrient = record
            .get(2)
            .expect("expected nutrient identifier at index2");
        let value = record.get(3).expect("expected value at index 3");
        // we use the lower value of the range instead of the range on fat.
        //let value = value.splitn(2, "-").next().expect("expected value at index 3").replacen(",", ".", 1);
        let value = value
            .splitn(2, "-")
            .next()
            .expect("expected value at index 3");

        let per = record.get(4).expect("per identifier at index 4");
        let nutrient = match parse_nutrient(nutrient, per, value) {
            Ok(x) => x,
            Err(NutrientError::Unknown(o)) => panic!(" unknown nutrient identifier: {o}"),
            Err(NutrientError::Skipped) => continue,
            // there are a lot of unknown values for infants
            Err(NutrientError::Underlying(o)) => {
                panic!("Underlying error occured: {o} in {:?}", record)
            }
        };
        let sex_key = match &sex {
            Sex::Male => "MALE",
            Sex::Female => "FEMALE",
        };
        let push_key = match &group {
            Group::Infants(start, ..) => format!("{sex_key}_{start}_MONTHS"),
            Group::Children(start, ..) | Group::Teenager(start, ..) | Group::Adults(start, ..) => {
                format!("{sex_key}_{start}_YEARS")
            }
        };
        let lkey = (sex, group, push_key);

        if let Some(entries) = lookup.get_mut(&lkey) {
            entries.push(nutrient);
        } else {
            lookup.insert(lkey, vec![nutrient]);
        }
    }
    let mut buffer = String::new();
    use std::fmt::Write;
    writeln!(&mut buffer, "#[rustfmt::skip]")?;
    writeln!(&mut buffer, "use voodplaner_core::*;")?;
    writeln!(&mut buffer, "use Sex::*;")?;
    writeln!(&mut buffer, "use Nutrient::*;")?;
    writeln!(&mut buffer, "use Amount::*;")?;
    writeln!(&mut buffer, "use VitaminCategory::*;")?;
    writeln!(&mut buffer, "")?;
    for ((_, _, k), v) in &lookup {
        writeln!(&mut buffer, "const {k}: [Nutrient; {}] = [", v.len())?;
        for v in v {
            writeln!(&mut buffer, "    {:?},", v)?;
        }
        writeln!(&mut buffer, "];")?;
        writeln!(&mut buffer, "")?;
    }

    writeln!(
        &mut buffer,
        "/// returns a reference to the nutrient table found by sex and starting age of the group"
    )?;
    writeln!(
        &mut buffer,
        "pub fn lookup(sex: &Sex, age: &Age) -> &'static [Nutrient] {{"
    )?;
    writeln!(&mut buffer, "    match (sex, age.months()) {{")?;
    for (s, a, k) in lookup.keys() {
        let ename = match &a {
            Group::Infants(start, end) => format!("{start}..={}", end -1),
            Group::Children(start, end) | 
            Group::Teenager(start, end) => format!("{}..={}", *start as u16 * 12, * end as u16 * 12 -1),
            Group::Adults(start, end) => format!("{}..={}", *start as u16 * 12, end.unwrap_or(250) as u16 * 12 -1),
        };
        writeln!(
            &mut buffer,
            "        ({:?}, {}) => &{},",
            s,
            ename,
            k
        )?;
    }
    writeln!(&mut buffer, "        _ => &[]")?;
    writeln!(&mut buffer, "")?;
    writeln!(&mut buffer, "    }}")?;
    writeln!(&mut buffer, "}}")?;

    Ok(buffer)
}

enum NutrientError {
    Unknown(String),
    Skipped,
    Underlying(Box<dyn Error>),
}

impl From<Box<dyn Error>> for NutrientError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::Underlying(value)
    }
}

fn parse_nutrient(nutrient: &str, per: &str, value: &str) -> Result<Nutrient, NutrientError> {
    Ok(match nutrient {
        "Alkohol" => Nutrient::Alcohol(parse_value(per, value)?),
        "Ballaststoffe" => Nutrient::Fiber(parse_value(per, value)?),
        "Biotin" => Nutrient::Biotin(parse_value(per, value)?),
        "Calcium" => Nutrient::Calcium(parse_value(per, value)?),
        "Chlorid" => Nutrient::Chloride(parse_value(per, value)?),
        "Chrom" => Nutrient::Chromium(parse_value(per, value)?),
        "Einfach ungesättigte Fettsäuren" => {
            Nutrient::MonounsaturatedFattyAcids(parse_value(per, value)?)
        }
        "Eisen" => Nutrient::Iron(parse_value(per, value)?),
        // ignore energie as we calculate it per individual instead of using a guide number
        "Energie" => return Err(NutrientError::Skipped),
        "EPA_DHA" => Nutrient::EpaDha(parse_value(per, value)?),
        "Fluorid" => Nutrient::Fluoride(parse_value(per, value)?),
        "Folat" => Nutrient::Folate(parse_value(per, value)?),
        "Gesamtfett" => Nutrient::TotalFat(parse_value(per, value)?),
        "Gesättigte Fettsäuren" => Nutrient::SaturatedFattyAcids(parse_value(per, value)?),
        "Jod" => Nutrient::Iodine(parse_value(per, value)?),
        "Kalium" => Nutrient::Potassium(parse_value(per, value)?),
        "Kohlenhydrate" => Nutrient::Carbohydrates(parse_value(per, value)?),
        "Kupfer" => Nutrient::Copper(parse_value(per, value)?),
        "Linolsäure" => Nutrient::LinoleicAcid(parse_value(per, value)?),
        "Magnesium" => Nutrient::Magnesium(parse_value(per, value)?),
        "Mangan" => Nutrient::Manganese(parse_value(per, value)?),
        "Mehrfach ungesättigte Fettsäuren" => {
            Nutrient::PolyunsaturatedFattyAcids(parse_value(per, value)?)
        }
        "Molybdän" => Nutrient::Molybdenum(parse_value(per, value)?),
        "Natrium" => Nutrient::Sodium(parse_value(per, value)?),
        "Niacin" => Nutrient::Niacin(parse_value(per, value)?),
        "Pantothensäure" => Nutrient::PantothenicAcid(parse_value(per, value)?),
        "Phosphor" => Nutrient::Phosphorus(parse_value(per, value)?),
        "Protein" => Nutrient::Protein(parse_value(per, value)?),
        "Riboflavin" => Nutrient::Riboflavin(parse_value(per, value)?),
        "Selen" => Nutrient::Selenium(parse_value(per, value)?),
        "Thiamin" => Nutrient::Thiamin(parse_value(per, value)?),
        "Vitamin A" => Nutrient::Vitamin(VitaminCategory::A, parse_value(per, value)?),
        "Vitamin B12 (Cobalamine)" => {
            Nutrient::Vitamin(VitaminCategory::B12, parse_value(per, value)?)
        }
        "Vitamin B6" => Nutrient::Vitamin(VitaminCategory::B6, parse_value(per, value)?),
        "Vitamin C" => Nutrient::Vitamin(VitaminCategory::C, parse_value(per, value)?),
        "Vitamin D" => Nutrient::Vitamin(VitaminCategory::D, parse_value(per, value)?),
        "Vitamin E" => Nutrient::Vitamin(VitaminCategory::E, parse_value(per, value)?),
        "Vitamin K" => Nutrient::Vitamin(VitaminCategory::K, parse_value(per, value)?),
        "Zink" => Nutrient::Zinc(parse_value(per, value)?),
        "α-Linolensäure" => Nutrient::AlphaLinolenicAcid(parse_value(per, value)?),

        err => return Err(NutrientError::Unknown(err.into())),
    })
}

