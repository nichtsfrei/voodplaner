mod parser {
    use std::{error, ops::Range};

    #[derive(Debug, PartialEq)]
    pub enum Field {
        Empty,
        Lookup(Range<usize>),
        // Text(String),
        // // maybe we should be more specific about numbers?
        // Float(f32),
    }

    #[derive(Debug, PartialEq)]
    pub struct Fields<'a> {
        pub line: &'a str,
        pub fields: Vec<Field>,
    }

    impl<'a> Fields<'a> {
        pub fn lookup(&self, index: usize) -> Option<&'a str> {
            self.fields.get(index).and_then(|x| match x {
                Field::Empty => None,
                Field::Lookup(x) => Some(&self.line[x.start..x.end]),
            })
        }
    }

    pub fn parse_line(line: &str) -> Fields {
        let mut is_text = false;
        let mut fields = Vec::with_capacity(2);
        let mut from = None;
        let mut bytes = line.bytes().enumerate();
        loop {
            let c = bytes.next();
            match c {
                Some((_, b'\n' | b'\r')) => continue,
                Some((_, b'~')) if !is_text => is_text = true,
                Some((_, b'~')) if is_text => is_text = false,
                Some((_, b'^')) | None => {
                    match from {
                        None => fields.push(Field::Empty),
                        Some((start, end)) => fields.push(Field::Lookup(start..end)),
                    }
                    is_text = false;
                    from = None;
                    if c.is_none() {
                        break;
                    }
                }

                Some((i, _)) => match from {
                    None => from = Some((i, i)),
                    Some((start, _)) => from = Some((start, i + 1)),
                },
            }
        }

        Fields { line, fields }
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn parse_lines() {
            let result = super::parse_line("~this is a string~^^23^2.01^~bla~");
            use super::Field::*;
            assert_eq!(
                result.fields,
                vec![
                    Lookup(1..17),
                    Empty,
                    Lookup(20..22),
                    Lookup(23..27),
                    Lookup(29..32)
                ]
            );
            assert_eq!(result.lookup(0), Some("this is a string"));
            assert_eq!(result.lookup(1), None);
            assert_eq!(result.lookup(2), Some("23"));
            assert_eq!(result.lookup(3), Some("2.01"));
            assert_eq!(result.lookup(4), Some("bla"));
        }

        #[test]
        fn hu() {
            let result = super::parse_line("~313~^~µg~^~FLD~");
            assert_eq!(result.lookup(0), Some("313"));
            assert_eq!(result.lookup(1), Some("µg"));
            assert_eq!(result.lookup(2), Some("FLD"));
        }
    }
}

mod groups {
    use voodplaner_core::IncredientGroup;
    const GROUP_IDENTIFIER: &'static str = include_str!("../data/FD_GROUP.txt");
    /// We use those groups to immediately reduce the amount of products as we just need to core
    /// ingredients.
    pub(crate) fn create_lookup_table() -> Vec<(&'static str, IncredientGroup)> {
        //~0200~^~Spices and Herbs~
        //~0400~^~Fats and Oils~
        //~0900~^~Fruits and Fruit Juices~
        //~1100~^~Vegetables and Vegetable Products~
        //~1200~^~Nut and Seed Products~
        //~1600~^~Legumes and Legume Products~
        //~2000~^~Cereal Grains and Pasta~
        let mut results = Vec::with_capacity(7);
        for l in GROUP_IDENTIFIER.lines() {
            let lu = crate::parser::parse_line(l);
            if let Some(g) = match lu.lookup(1) {
                Some("Spices and Herbs") => Some(IncredientGroup::SpiceAndHerb),
                Some("Fats and Oils") => Some(IncredientGroup::FatAndOil),
                Some("Fruits and Fruit Juices") => Some(IncredientGroup::Fruit),
                Some("Vegetables and Vegetable Products") => Some(IncredientGroup::Vegetable),
                Some("Nut and Seed Products") => Some(IncredientGroup::Nut),
                Some("Legumes and Legume Products") => Some(IncredientGroup::Legume),
                Some("Cereal Grains and Pasta") => Some(IncredientGroup::Grain),
                Some(_) => None,
                None => None,
            } {
                results.push((lu.lookup(0).unwrap(), g));
            }
        }
        results
    }
    #[cfg(test)]
    mod tests {

        #[test]
        fn filter_groups() {
            let wanted_groups = super::create_lookup_table();
            use voodplaner_core::IncredientGroup::*;
            assert_eq!(
                wanted_groups,
                vec![
                    ("0200", SpiceAndHerb),
                    ("0400", FatAndOil),
                    ("0900", Fruit),
                    ("1100", Vegetable),
                    ("1200", Nut),
                    ("1600", Legume),
                    ("2000", Grain)
                ]
            );
        }
    }
}

mod nutrient {
    use crate::parser::Fields;

    const DATA: &'static str = include_str!("../data/NUT_DATA.txt");
    const DEF: &'static str = include_str!("../data/NUTR_DEF.txt");
    #[derive(Debug, Clone)]
    pub(crate) struct Definition<'a> {
        nutr_no: &'a str,
        units: &'a str,
        tag_name: Option<Vec<voodplaner_core::infood::Tag>>,
        name: &'a str,
    }

    fn lookup_infood_tag(f: &Fields) -> Vec<voodplaner_core::infood::Tag> {
        match f.lookup(2) {
            Some("") | None => {
                match f.lookup(0).expect("food tag") {
                    // TODO FIX ME
                    // adjusted protein
                    "257" => vec![voodplaner_core::infood::Tag::PROT],
                    // phosphorus
                    "305" => vec![voodplaner_core::infood::Tag::P],
                    // potassium
                    "306" => vec![voodplaner_core::infood::Tag::K],
                    "573" => vec![voodplaner_core::infood::Tag::VITE],
                    "578" => vec![voodplaner_core::infood::Tag::VITB12],
                    // not further defined
                    "665" => vec![],
                    "666" => vec![],
                    "856" => vec![],
                    _ => panic!(
                        "lookup:{} -> {}",
                        f.lookup(0).unwrap(),
                        f.lookup(3).unwrap(),
                    ),
                }
            }
            Some("FLD") => vec![voodplaner_core::infood::Tag::FD],
            Some("LUT+ZEA") => vec![
                voodplaner_core::infood::Tag::LUTN,
                voodplaner_core::infood::Tag::ZEA,
            ],
            Some("TRP_G") => vec![voodplaner_core::infood::Tag::TRP],
            Some("THR_G") => vec![voodplaner_core::infood::Tag::THR],
            Some("F22D1T") => vec![voodplaner_core::infood::Tag::F22D1],
            Some("F22D1C") => vec![voodplaner_core::infood::Tag::F22D1],
            Some("F18D1TN7") => vec![voodplaner_core::infood::Tag::F18D1N7],
            Some("F18D2TT") => vec![voodplaner_core::infood::Tag::F18D2],
            Some("F18D2CLA") => vec![voodplaner_core::infood::Tag::F18D2],
            Some("F18D3CN6") => vec![voodplaner_core::infood::Tag::F18D3N6],
            Some("F21D5") => vec![voodplaner_core::infood::Tag::F21D5N3],
            Some(t) => {
                let t = voodplaner_core::infood::lookup(t);
                if t.is_empty() {
                    panic!(
                        "lookup:{} -> {} => {}",
                        f.lookup(0).unwrap(),
                        f.lookup(3).unwrap(),
                        f.lookup(2).unwrap()
                    )
                }
                t.into_iter().map(|(t, _)| t).collect()
            }
        }
    }

    pub(crate) fn definitions<'a>() -> impl Iterator<Item = Definition<'a>> + 'a {
        DEF.lines().map(crate::parser::parse_line).map(|l| {
            let nutr_no = l.lookup(0).expect("nutr_no");

            Definition {
                nutr_no: nutr_no,
                units: l.lookup(1).expect("units"),
                tag_name: Some(lookup_infood_tag(&l)),
                name: l.lookup(3).expect("desc"),
            }
        })
    }

    #[derive(Debug, Clone)]
    pub(crate) struct Data<'a> {
        definition: &'a Definition<'a>,
        val: &'a str,
        added: Option<&'a str>,
    }

    pub(crate) fn nutrients<'a>(
        lookup: &'a [Definition<'a>],
    ) -> impl Iterator<Item = (&'a str, Data<'a>)> + 'a {
        let definition = |k| {
            lookup
                .iter()
                .find(|x| x.nutr_no == k)
                .expect("expected definition")
        };
        DATA.lines().map(crate::parser::parse_line).map(move |x| {
            let ndb_no = x.lookup(0).expect("ndb_no");
            let definition = definition(x.lookup(1).expect("nutr_no"));
            let val = x.lookup(2).expect("nutr_val");
            let added = x.lookup(8);
            (
                ndb_no,
                Data {
                    definition,
                    val,
                    added,
                },
            )
        })
    }
}

mod description {
    use std::collections::HashMap;

    use voodplaner_core::{IncredientGroup, Nutrient};
    const DESCRIPTOR: &'static str = include_str!("../data/FOOD_DES.txt");
    enum Reference {}
    struct Description {
        name: String,
        group: IncredientGroup,
        manufactor: Option<String>,
        nutrients: Vec<Nutrient>,
        references: Vec<Reference>,
    }
    pub fn create_lookup_table() -> Vec<Description> {
        #[derive(Debug, Clone, PartialEq)]
        struct KeyInformation {
            id: String,
            name: String,
            group: IncredientGroup,
            n_factor: Option<String>,
        }

        fn is_in_group<'a>(
            id: &'a str,
            groups: &'a [(&'static str, IncredientGroup)],
        ) -> Option<&'a IncredientGroup> {
            for (g, x) in groups {
                if g == &id {
                    return Some(x);
                }
            }
            None
        }

        // guessing game
        fn does_contain_unwanted_marker(name: &str) -> bool {
            let blackwords = [
                "MILK",
                "BEEF",
                "CHICKEN",
                "DUCK",
                "GOOSE",
                "POULTRY",
                "LARD",
                "TURKEY",
                "FISH",
                "ANIMAL FAT",
                "BUTTER",
                "MARGARINE-LIKE",
                "HVY SYRUP",
                "MAYONNAISE",
                "DRSNG",
                // can have up to 10% milk fat
                "MARGARINE",
                "SANDWICH",
                "POTATO SALAD",
                "SUCCOTASH",
                "PUDDING",
                "CHEESE",
                "ONION RINGS",
            ];
            for b in blackwords {
                if name.contains(b) {
                    return true;
                }
            }
            return false;
        }
        // TODO create a closure factory to easier use iter map instead of for
        fn parse_line(
            l: &str,
            groups: &[(&'static str, IncredientGroup)],
        ) -> Option<KeyInformation> {
            let lu = crate::parser::parse_line(l);
            let group = lu.lookup(1)?;
            let group = is_in_group(group, groups)?;
            //let name = lu.lookup(9)?;
            let name = match lu.lookup(9) {
                Some(x) => x,
                None => lu.lookup(3).unwrap(),
            };
            if does_contain_unwanted_marker(name) {
                return None;
            }

            if lu.lookup(5).is_some() {
                // although we will have products incredients later on we don't
                // usda products as we cannot verify if it is vegan.
                return None;
            }

            let id = lu.lookup(0).expect("expected id").to_string();
            let group = group.clone();
            let name = name.to_string();
            let n_factor = lu.lookup(10).map(|x| x.to_string());
            Some(KeyInformation {
                id,
                name,
                group,
                n_factor,
            })
        }
        let groups_to_include = crate::groups::create_lookup_table();
        let mut results = Vec::with_capacity(1744);
        for l in DESCRIPTOR.lines() {
            if let Some(r) = parse_line(l, &groups_to_include) {
                results.push(r);
            }
        }

        let nutr_definitions =
            crate::nutrient::definitions().collect::<Vec<crate::nutrient::Definition<'_>>>();
        let nutr_data: HashMap<_, _> = crate::nutrient::nutrients(&nutr_definitions).collect();
        dbg!(nutr_data.len());

        // fist we filter out nutrient data by the already used id
        // then we remove everything that has `ADd_Nutr_Mark` as it is processed and we don't care.
        // ignoring LANGUA for now, but got potentially be used to filter further
        vec![]
    }
    #[cfg(test)]
    mod tests {

        #[test]
        fn filter_groups() {
            let foods = super::create_lookup_table();
            assert_eq!(foods.len(), 1744);
        }
    }
}
