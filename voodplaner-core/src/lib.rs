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

impl Age {
    pub fn months(&self) -> u16 {
        match self {
            Age::Months(x) => *x as u16,
            Age::Years(x) => *x as u16 * 12,
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
    KCal(f32),
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amount::PercentageOfEnergy(value) => write!(f, "{}% of energy", value),
            Amount::GrammPerWeight(value) => write!(f, "{} g/kg", value),
            Amount::GrammPerDay(value) => write!(f, "{} g/day", value),
            Amount::MilliGramPerDay(value) => write!(f, "{} mg/day", value),
            Amount::MicroGramPerDay(value) => write!(f, "{} µg/day", value),
            Amount::KCal(value) => write!(f, "{} kcal", value),
        }
    }
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
    Energy(Amount, Vec<Nutrient>),
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
impl Nutrient {
    /// Returns true when the Nutrient is considered unhealthy.
    ///
    /// This is the case when it's a nutrient that should be limited in a diet.
    /// Consider the value as a maximum amount, not a recommendation.
    pub fn unhealthy(&self) -> bool {
        match self {
            Nutrient::Alcohol(_) => true,
            Nutrient::SaturatedFattyAcids(_) => true,
            Nutrient::Sodium(_) => true,
            _ => false,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Nutrient::Alcohol(_) => "Ethanol",
            Nutrient::Fiber(_) => "Dietary Fiber",
            Nutrient::Biotin(_) => "Biotin",
            Nutrient::Calcium(_) => "Calcium",
            Nutrient::Chloride(_) => "Chloride",
            Nutrient::Chromium(_) => "Chromium",
            Nutrient::MonounsaturatedFattyAcids(_) => "Monounsaturated Fatty Acids",
            Nutrient::Iron(_) => "Iron",
            Nutrient::Energy(_, _) => "Energy",
            Nutrient::EpaDha(_) => "Eicosapentaenoic and Docosahexaenoic Acid",
            Nutrient::Fluoride(_) => "Fluoride",
            Nutrient::Folate(_) => "Folate",
            Nutrient::TotalFat(_) => "Total Fat",
            Nutrient::SaturatedFattyAcids(_) => "Saturated Fatty Acids",
            Nutrient::Iodine(_) => "Iodine",
            Nutrient::Potassium(_) => "Potassium",
            Nutrient::Carbohydrates(_) => "Carbohydrates",
            Nutrient::Copper(_) => "Copper",
            Nutrient::LinoleicAcid(_) => "Linoleic Acid",
            Nutrient::Magnesium(_) => "Magnesium",
            Nutrient::Manganese(_) => "Manganese",
            Nutrient::PolyunsaturatedFattyAcids(_) => "Polyunsaturated Fatty Acids",
            Nutrient::Molybdenum(_) => "Molybdenum",
            Nutrient::Sodium(_) => "Sodium",
            Nutrient::Niacin(_) => "Niacin",
            Nutrient::PantothenicAcid(_) => "Pantothenic Acid",
            Nutrient::Phosphorus(_) => "Phosphorus",
            Nutrient::Protein(_) => "Protein",
            Nutrient::Riboflavin(_) => "Riboflavin",
            Nutrient::Selenium(_) => "Selenium",
            Nutrient::Thiamin(_) => "Thiamin",
            Nutrient::Vitamin(_, _) => "Vitamin",
            Nutrient::Zinc(_) => "Zinc",
            Nutrient::AlphaLinolenicAcid(_) => "Alpha-Linolenic Acid",
        }
    }

    pub fn amount(&self) -> &Amount {
        match self {
            Self::Alcohol(amount)
            | Self::Fiber(amount)
            | Self::Biotin(amount)
            | Self::Calcium(amount)
            | Self::Chloride(amount)
            | Self::Chromium(amount)
            | Self::MonounsaturatedFattyAcids(amount)
            | Self::Iron(amount)
            | Self::Energy(amount, _)
            | Self::EpaDha(amount)
            | Self::Fluoride(amount)
            | Self::Folate(amount)
            | Self::TotalFat(amount)
            | Self::SaturatedFattyAcids(amount)
            | Self::Iodine(amount)
            | Self::Potassium(amount)
            | Self::Carbohydrates(amount)
            | Self::Copper(amount)
            | Self::LinoleicAcid(amount)
            | Self::Magnesium(amount)
            | Self::Manganese(amount)
            | Self::PolyunsaturatedFattyAcids(amount)
            | Self::Molybdenum(amount)
            | Self::Sodium(amount)
            | Self::Niacin(amount)
            | Self::PantothenicAcid(amount)
            | Self::Phosphorus(amount)
            | Self::Protein(amount)
            | Self::Riboflavin(amount)
            | Self::Selenium(amount)
            | Self::Thiamin(amount)
            | Self::Vitamin(_, amount)
            | Self::Zinc(amount)
            | Self::AlphaLinolenicAcid(amount) => amount,
        }
    }

    pub fn with_amount(&self, value: Amount) -> Nutrient {
        match self {
            Self::Vitamin(x, _) => Self::Vitamin(x.clone(), value),
            Self::Energy(_, x) => Self::Energy(value, x.to_vec()),
            Self::Alcohol(_) => Self::Alcohol(value),
            Self::Fiber(_) => Self::Fiber(value),
            Self::Biotin(_) => Self::Biotin(value),
            Self::Calcium(_) => Self::Calcium(value),
            Self::Chloride(_) => Self::Chloride(value),
            Self::Chromium(_) => Self::Chromium(value),
            Self::MonounsaturatedFattyAcids(_) => Self::MonounsaturatedFattyAcids(value),
            Self::Iron(_) => Self::Iron(value),
            Self::EpaDha(_) => Self::EpaDha(value),
            Self::Fluoride(_) => Self::Fluoride(value),
            Self::Folate(_) => Self::Folate(value),
            Self::TotalFat(_) => Self::TotalFat(value),
            Self::SaturatedFattyAcids(_) => Self::SaturatedFattyAcids(value),
            Self::Iodine(_) => Self::Iodine(value),
            Self::Potassium(_) => Self::Potassium(value),
            Self::Carbohydrates(_) => Self::Carbohydrates(value),
            Self::Copper(_) => Self::Copper(value),
            Self::LinoleicAcid(_) => Self::LinoleicAcid(value),
            Self::Magnesium(_) => Self::Magnesium(value),
            Self::Manganese(_) => Self::Manganese(value),
            Self::PolyunsaturatedFattyAcids(_) => Self::PolyunsaturatedFattyAcids(value),
            Self::Molybdenum(_) => Self::Molybdenum(value),
            Self::Sodium(_) => Self::Sodium(value),
            Self::Niacin(_) => Self::Niacin(value),
            Self::PantothenicAcid(_) => Self::PantothenicAcid(value),
            Self::Phosphorus(_) => Self::Phosphorus(value),
            Self::Protein(_) => Self::Protein(value),
            Self::Riboflavin(_) => Self::Riboflavin(value),
            Self::Selenium(_) => Self::Selenium(value),
            Self::Thiamin(_) => Self::Thiamin(value),
            Self::Zinc(_) => Self::Zinc(value),
            Self::AlphaLinolenicAcid(_) => Self::AlphaLinolenicAcid(value),
        }
    }

    /// Returns true when the Nutrient is considered healthy.
    ///
    /// Consider the value as a recommendated amount.
    pub fn healthy(&self) -> bool {
        !self.unhealthy()
    }
}
