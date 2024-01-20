use std::fmt::Write;

const PART1: &'static [u8] = include_bytes!("../data/PART1.TXT");
const PART2: &'static [u8] = include_bytes!("../data/PART2.TXT");
const PART3: &'static [u8] = include_bytes!("../data/PART3.TXT");
const PART4: &'static [u8] = include_bytes!("../data/PART4.TXT");
const PART5: &'static [u8] = include_bytes!("../data/PART5.TXT");

const ADDITION2008: &'static str = include_str!("../data/TAGREV__1_.csv");
const ADDITION2010: &'static str = include_str!("../data/Tagname_new_April_2010-web__2_.csv");

#[derive(Debug, Clone)]
pub enum Error {
    UnknownUnit,
}
/// Structure of newer CSV
///
/// For parts given in this txt format it is not necessarily true but at leat unit and short
/// description is available everywhere. The tables keyword is ignored
#[derive(Debug, Clone, PartialEq, Default)]
struct InFoodTag {
    tag: String,
    unit: String,
    short_description: String,
    description: String,
    comment: String,
    synonyms: String,
}

impl InFoodTag {
    fn normalize(input: &str) -> String {
        let reducer = |a: String, b: &str| {
            if a.is_empty() {
                b.to_string()
            } else if b.is_empty() {
                a.to_string()
            } else {
                format!("{a} {b}")
            }
        };

        let output = input.split('\n').fold(String::new(), reducer);
        let output = output.split(' ').fold(String::new(), reducer);

        output.to_string()
    }

    fn tag(&self) -> String {
        Self::normalize(&self.tag)
    }
    fn unit(&self) -> String {
        Self::normalize(&self.unit)
    }
    fn short_description(&self) -> String {
        Self::normalize(&self.short_description)
    }
    fn description(&self) -> String {
        Self::normalize(&self.description)
    }
    fn comment(&self) -> String {
        Self::normalize(&self.comment)
    }
    fn synonyms(&self) -> String {
        Self::normalize(&self.synonyms)
    }
}

fn parse_part(known: &[&str], part: &[u8]) -> Vec<InFoodTag> {
    #[derive(Debug, Clone, Copy)]
    enum KeyWord {
        Description,
        Unit,
        Comment,
        Synonym,
    }
    #[derive(Clone, Debug)]
    enum State {
        Searching(usize),
        Skip(usize),
        MayKeyword(usize, Box<State>),
        FoundKeyword(KeyWord),
        InTag(usize),
        // special because not keyword based
        SearchShortDescription,
        InKeyWord(KeyWord, usize, usize, usize),
    }
    let mut tags = Vec::with_capacity(300);
    let mut state = State::Searching(0);

    use State::*;
    for (i, c) in part.iter().map(|x| *x as char).enumerate() {
        match (&state, c) {
            (Searching(_), '\n' | '\r') => state = Searching(0),
            (Searching(0), '<') => {
                tags.push(InFoodTag::default());
                state = InTag(i + 1);
            }
            (Searching(i), ' ' | '\t') => state = Searching(i + 1),
            (Searching(ws), c) if ws > &4 && c.is_uppercase() => {
                state = MayKeyword(i, Box::new(Skip(0)))
            }

            (InTag(start), '>') => {
                let tag = std::str::from_utf8(&part[*start..i]).unwrap();

                if in_known(known, tag) {
                    state = Skip(0);
                } else {
                    write!(tags.last_mut().unwrap().tag, "{tag}").unwrap();
                    state = SearchShortDescription;
                }
            }
            (InTag(_), _) => {}

            (SearchShortDescription, ' ' | '\t') => {}
            (SearchShortDescription, '\n' | '\r') => {}
            (SearchShortDescription, _) => state = InKeyWord(KeyWord::Description, i, 0, 0),

            (Skip(0), '\n' | '\r') => state = Searching(0),
            (Skip(_), '\n' | '\r') => state = Skip(0),
            (Skip(i), _) => state = Skip(i + 1),

            (MayKeyword(start, _), ':') => {
                let kw = &part[*start..i];
                let kw = std::str::from_utf8(kw).unwrap();
                state = match kw {
                    "Unit" => FoundKeyword(KeyWord::Unit),
                    "Comments" => FoundKeyword(KeyWord::Comment),
                    "Synonyms" => FoundKeyword(KeyWord::Synonym),
                    _ => Skip(0),
                }
            }

            (MayKeyword(_, jb), ' ' | '\t') | (MayKeyword(_, jb), '\n' | '\r') => {
                state = jb.as_ref().clone()
            }
            (FoundKeyword(s), ' ' | '\t') => {
                state = InKeyWord(*s, i + 1, 1, 1);
            }

            (InKeyWord(_, _, 0, 0), '<') => {
                tags.push(InFoodTag::default());
                state = InTag(i + 1);
            }
            (InKeyWord(kw, start, _, _), '\n' | '\r') => state = InKeyWord(*kw, *start, 0, 0),
            // skip starting ws
            (InKeyWord(kw, start, ws, 0), ' ' | '\t') => state = InKeyWord(*kw, *start, ws + 1, 0),
            // for the case it starts with an uppercase we simply ignore the rest
            (InKeyWord(kw, start, _, 0), c) if c.is_uppercase() => {
                state = MayKeyword(i, Box::new(InKeyWord(*kw, *start, 1, 0)))
            }
            (InKeyWord(kw, start, ws, cc), _) => {
                let writeable = {
                    match kw {
                        KeyWord::Description => &mut tags.last_mut().unwrap().short_description,
                        KeyWord::Unit => &mut tags.last_mut().unwrap().unit,
                        KeyWord::Comment => &mut tags.last_mut().unwrap().comment,
                        KeyWord::Synonym => &mut tags.last_mut().unwrap().synonyms,
                    }
                };

                if let Ok(c) = std::str::from_utf8(&part[*start..i + 1]) {
                    write!(writeable, "{}", c.replace('\r', "")).unwrap();
                }
                state = InKeyWord(*kw, i + 1, ws + 1, cc + 1);
            }

            _ => {}
        }
    }
    tags
}

fn in_known<T>(known: &[T], e: &str) -> bool
where
    T: AsRef<str>,
{
    known.iter().find(|x| x.as_ref() == e).is_some()
}

fn parse_csv(known: &[&str], data: &str) -> Vec<InFoodTag> {
    let mut duplicates = Vec::with_capacity(4);
    let mut rdr = csv::Reader::from_reader(std::io::Cursor::new(data));
    let mut records = rdr.records();
    let mut results = Vec::with_capacity(100);
    // ignore header definitions
    let _ = records.next();
    for r in records {
        // "TAGNAME","Short description","Description","Recommended units","Comment","SYNONYMS"
        let record = r.unwrap();
        let tag = record.get(0).unwrap().to_string();
        if in_known(known, &tag) {
            continue;
        }
        if in_known(&duplicates, &tag) {
            continue;
        }
        duplicates.push(tag.clone());
        let short_description = record.get(1).unwrap().to_string();
        let description = record.get(2).unwrap().to_string();
        if description.contains("fish") || description.contains("Fish") {
            continue;
        }

        let unit = record.get(3).unwrap().to_string();
        let comment = record.get(4).unwrap().to_string();
        let synonyms = record.get(5).unwrap().to_string();

        let tag = InFoodTag {
            tag,
            unit,
            short_description,
            description,
            comment,
            synonyms,
        };

        results.push(tag);
    }
    results
}

pub fn generate() -> String {
    fn normalize_group(g: &str) -> &str {
        if g.contains("nitrogen") {
            return "NT";
        }
        if g.contains("fatty acid") {
            return "FACIS";
        }
        if g.contains("total fatty") {
            return "FAT";
        }
        if g.contains("protein") {
            return "PROT";
        }
        g
    }
    fn normalize_unit(u: &str) -> String {
        fn single(u: &str) -> &str {
            match u {
                "?" => "NoUnitAvailable",
                "%" => "Percent",
                "g" => "Gramm",
                "mg" => "MilliGramm",
                "mcg" => "MicroGramm",
                "kJ" => "KiloJule",
                "100 g" | "100g" => "Fix(100.0, Box::new(Gramm))",
                "100" => "Fix(100.0, Box::new(Gramm))",
                "None" => "NoUnitAvailable",
                "IU" => "InternationalUnit",
                "mass per unit volume" => "MassPerUnitVolume",
                "kcal" => "KiloCalories",
                _ => u,
            }
        }

        let mut su = u.trim();
        if su.starts_with("%") {
            su = "%";
        }

        if su.starts_with("None") || su.is_empty() || su.starts_with("none") {
            su = "None";
        }
        if su.starts_with("kJ") {
            su = "kJ";
        }
        if su.starts_with("mass per unit volume") {
            su = "mass per unit volume";
        }
        if !su.contains("/>") {
            if let Some((divident, divisor)) = su.split_once('/') {
                let divident = single(divident);
                let aha = divisor.splitn(2, ' ').collect::<Vec<_>>();

                return match &aha[..] {
                    [a, "g"] | [a] => {
                        format!("Per(Box::new({}), Box::new({}))", divident, single(a))
                    }
                    // except when b is 'g'
                    [a, b] => format!(
                        "PerGroup(Box::new({}), Box::new({}), {})",
                        divident,
                        single(a),
                        normalize_group(b)
                    ),
                    _ => unreachable!(),
                };
            }
        }
        if su.starts_with("mcg") {
            su = "mcg";
        }

        if su.starts_with("mg.") || su.starts_with("mg?") {
            su = "mg";
        }
        single(su).to_string()
    }

    // TAG, Group,Unit
    //    let mut tag: Vec<(&str, &str, &str)> = Vec::with_capacity(1000);
    let mut tag = parse_csv(&[], ADDITION2010);
    tag.extend_from_slice(&parse_csv(
        &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
        ADDITION2008,
    ));
    tag.extend_from_slice(&parse_part(
        &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
        PART1,
    ));
    tag.extend_from_slice(&parse_part(
        &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
        PART2,
    ));
    tag.extend_from_slice(&parse_part(
        &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
        PART3,
    ));
    tag.extend_from_slice(&parse_part(
        &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
        PART4,
    ));
    tag.extend_from_slice(&parse_part(
        &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
        PART5,
    ));
    let mut result = String::new();
    let as_enum = |s: &str| {
        let first_non_digit = s
            .chars()
            .position(|c| !c.is_digit(10))
            .unwrap_or_else(|| s.len());

        let (first_part, second_part) = s.split_at(first_non_digit);
        let s = format!("{}{}", second_part, first_part);
        s.replace('-', "Minus").replace('+', "Plus")
    };
    writeln!(result, "// Contains known infood tags generated from:").unwrap();
    writeln!(result, "// https://www.fao.org/infoods/infoods/standards-guidelines/food-component-identifiers-tagnames/en/").unwrap();
    writeln!(result, "").unwrap();

    writeln!(result, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]").unwrap();
    writeln!(result, "pub enum Tag {{").unwrap();
    for t in &tag {
        let tag = t.tag();
        if tag.is_empty() {
            // The file parser is buggy af and doesn't skip completely
            // on double entry hence it can happen that an empty tag slipps through
            continue;
        }
        let sd = t.short_description();
        let d = t.description();
        let comment = t.comment();
        let synonyms = t.synonyms();
        let unit = t.unit();
        writeln!(result, "    /// {} ({})", tag, unit).unwrap();
        writeln!(result, "    ///").unwrap();
        if !sd.is_empty() {
            writeln!(result, "    /// # Short Description").unwrap();
            writeln!(result, "    /// {}", sd).unwrap();
        }
        if !d.is_empty() {
            writeln!(result, "    /// # Description").unwrap();
            writeln!(result, "    /// {}", d).unwrap();
        }
        if !comment.is_empty() {
            writeln!(result, "    /// # Comment").unwrap();
            writeln!(result, "    /// {}", comment).unwrap();
        }
        if !synonyms.is_empty() {
            writeln!(result, "    /// # Synonyms").unwrap();
            writeln!(result, "    /// {}", synonyms).unwrap();
        }
        writeln!(result, "    {},", as_enum(&tag)).unwrap();
    }
    writeln!(result, "}}").unwrap();
    writeln!(result, "#[derive(Debug,Clone, PartialEq)]").unwrap();
    writeln!(result, "pub struct ParseError(String);").unwrap();
    writeln!(result, "").unwrap();
    writeln!(result, "impl std::fmt::Display for ParseError {{").unwrap();
    writeln!(
        result,
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
    )
    .unwrap();
    writeln!(result, "        write!(f, \"{{}}\", self.0)").unwrap();
    writeln!(result, "    }}").unwrap();
    writeln!(result, "}}").unwrap();
    writeln!(result, "").unwrap();
    writeln!(result, "impl TryFrom<&str> for Tag {{").unwrap();
    writeln!(result, "    type Error = ParseError;").unwrap();
    writeln!(result, "").unwrap();
    writeln!(
        result,
        "    fn try_from(value: &str) -> Result<Self, Self::Error> {{"
    )
    .unwrap();
    writeln!(result, "            use Tag::*;").unwrap();
    writeln!(result, "            match value {{").unwrap();
    writeln!(result, "").unwrap();
    for t in &tag {
        let tag = t.tag();
        if tag.is_empty() {
            // The file parser is buggy af and doesn't skip completely
            // on double entry hence it can happen that an empty tag slipps through
            continue;
        }

        writeln!(
            result,
            "                \"{}\" => Ok({}),",
            &tag,
            as_enum(&tag)
        )
        .unwrap();
    }
    writeln!(
        result,
        "                _ => Err(ParseError(format!(\"Unknown: {{value}}\"))),"
    )
    .unwrap();
    writeln!(result, "            }}").unwrap();
    writeln!(result, "    }}").unwrap();
    writeln!(result, "}}").unwrap();
    writeln!(result, "").unwrap();
    writeln!(result, "").unwrap();
    writeln!(result, "impl Into<Unit> for Tag {{").unwrap();
    writeln!(result, "    fn into(self) -> Unit {{").unwrap();
    writeln!(result, "        use Tag::*;").unwrap();
    writeln!(result, "        use Unit::*;").unwrap();
    writeln!(result, "        match self {{").unwrap();

    for t in &tag {
        let et = as_enum(&t.tag());
        if et.is_empty() {
            continue;
        }
        let unit = normalize_unit(&t.unit());
        writeln!(result, "                {} => {},", et, unit).unwrap();
    }

    writeln!(result, "        }}").unwrap();
    writeln!(result, "    }}").unwrap();
    writeln!(result, "}}").unwrap();
    writeln!(result, "").unwrap();
    writeln!(result, "").unwrap();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cvs() {
        let mut tag = parse_csv(&[], ADDITION2010);
        tag.extend_from_slice(&parse_csv(
            &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
            ADDITION2008,
        ));
        insta::assert_debug_snapshot!(tag);
    }

    #[test]
    fn parse_parts() {
        let mut tag = parse_part(&[], PART1);
        tag.extend_from_slice(&parse_part(
            &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
            PART2,
        ));
        tag.extend_from_slice(&parse_part(
            &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
            PART3,
        ));
        tag.extend_from_slice(&parse_part(
            &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
            PART4,
        ));
        tag.extend_from_slice(&parse_part(
            &tag.iter().map(|x| &x.tag as &str).collect::<Vec<_>>(),
            PART5,
        ));
        insta::assert_debug_snapshot!(tag);
    }
    
    #[test]
    fn generate() {
        insta::assert_snapshot!(super::generate());

    }
}
