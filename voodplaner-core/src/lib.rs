#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Group {
    Infants(u8, u8),
    Children(u8, u8),
    Teenager(u8, u8),
    Adults(u8, Option<u8>),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Age {
    Months(u8),
    Years(u8),
}

impl Age{
    pub fn months(&self) -> u16 {
        match self {
            Age::Months(x) => *x as u16,
            Age::Years(x) => *x as u16* 12,
        }

    }

    pub fn years(&self) -> u8 {
        match self {
            Age::Months(x) => *x / 12,
            Age::Years(x) => *x,
        }

    }
}

impl Group {
    pub fn age_keyword(&self) -> &'static str {
        match self {
            Group::Infants(..) => "MONTHS",
            Group::Children(..) | Group::Teenager(..) | Group::Adults(..) => "YEARS",
        }
    }
}
impl Group {
    pub fn start(&self) -> u8 {
        match self {
            Group::Infants(start, _)
            | Group::Children(start, _)
            | Group::Teenager(start, _)
            | Group::Adults(start, _) => *start,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Sex {
    Male,
    Female,
}

impl Sex {
    pub fn keyword(&self) -> &'static str {
        match self {
            Sex::Male => "MALE",
            Sex::Female => "FEMALE",
        }
    }
}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sex::Male => write!(f, "male"),
            Sex::Female => write!(f, "female"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum VitaminCategory {
    A,
    B12,
    B6,
    C,
    D,
    E,
    K,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Amount {
    PercentageOfEnergy(f32),
    GrammPerWeight(f32),
    GrammPerDay(f32),
    MilliGramPerDay(f32),
    MicroGramPerDay(f32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Nutrient {
    Alcohol(Amount),
    Fiber(Amount),
    Biotin(Amount),
    Calcium(Amount),
    Chloride(Amount),
    Chromium(Amount),
    MonounsaturatedFattyAcids(Amount),
    Iron(Amount),
    //Energy(Amount), we calculate it based on our own method, hence we ignore it
    EpaDha(Amount),
    Fluoride(Amount),
    Folate(Amount),
    TotalFat(Amount),
    SaturatedFattyAcids(Amount),
    Iodine(Amount),
    Potassium(Amount),
    Carbohydrates(Amount),
    Copper(Amount),
    LinoleicAcid(Amount),
    Magnesium(Amount),
    Manganese(Amount),
    PolyunsaturatedFattyAcids(Amount),
    Molybdenum(Amount),
    Sodium(Amount),
    Niacin(Amount),
    PantothenicAcid(Amount),
    Phosphorus(Amount),
    Protein(Amount),
    Riboflavin(Amount),
    Selenium(Amount),
    Thiamin(Amount),
    Vitamin(VitaminCategory, Amount),
    Zinc(Amount),
    AlphaLinolenicAcid(Amount),
}
#[derive(Debug, PartialEq, Clone)]
pub struct Recommandation {
    pub group: Group,
    pub sex: Sex,
    pub nutrient: Nutrient,
}
