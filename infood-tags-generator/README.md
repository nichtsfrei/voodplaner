# infood-tags-generator

Uses the information available on:
- https://www.fao.org/infoods/infoods/standards-guidelines/food-component-identifiers-tagnames/en/

copied into data dir to transform it to a enum that is included into `voodplaner-core`.

The enum is defined as 

```rust
// Contains known infood tags generated from:
// https://www.fao.org/infoods/infoods/standards-guidelines/food-component-identifiers-tagnames/en/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    /// AAE9 (mg)
    ///
    /// # Short Description
    /// AA 9 ess. incl. HIS
    /// # Description
    /// amino acids, total essential (9)
    /// # Comment
    /// mg
    /// # Synonyms
    /// mg
    AAE9,
    /// AAT19 (mg)
    ///
    /// # Short Description
    /// AA 19
    /// # Description
    /// sum of 19 amino acids (excluding tryptophan)
    /// # Comment
    /// mg
    /// # Synonyms
    /// mg
    AAT19,
    /// AAT20 (mg)
    ///
    /// # Short Description
    /// AA 20
    /// # Description
    /// sum of 20 amino acids
    /// # Comment
    /// mg
    /// # Synonyms
    /// mg
    AAT20,
    /// AAT24 (mg)
    ///
    /// # Short Description
    /// AA 24
    /// # Description
    /// amino acids, total
    /// # Comment
    /// mg
    /// # Synonyms
    /// mg
    AAT24,
    ...
    /// VALP (mg/100 g protein)
    ///
    /// # Short Description
    /// valine; expressed per quantity of protein
    VALP,
}
```

It also generates the possibility to transform from `&str` to `Tag`:
```
#[derive(Debug,Clone, PartialEq)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Tag {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
            use Tag::*;
            match value {
                "AAE9" => Ok(AAE9),
                "AAT19" => Ok(AAT19),
                ...
                "VALP" => Ok(VALP),
                _ => Err(ParseError(format!("Unknown: {value}"))),
            }
    }
}
```

And companion enum `Unit` to lookup unit definitions:
```
#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    NoUnitAvailable,
    MilliGramm,
    MicroGramm,
    Gramm,
    Percent,
    KiloJule,
    KiloCalories,
    Per(Box<Unit>,Box<Unit>),
    PerGroup(Box<Unit>,Box<Unit>, Tag),
    Fix(f32,Box<Unit>),
    MassPerUnitVolume,

}

impl Into<Unit> for Tag {
    fn into(self) -> Unit {
        use Tag::*;
        use Unit::*;
        match self {
                AAE9 => MilliGramm,
                ANTCYAN => MicroGramm,
                CAPSA => NoUnitAvailable,
                FAPULC => Gramm,
                XALC => KiloJule,
                REFUSE => Percent,
                RUBA => Per(Box::new(KiloCalories), Box::new(Gramm)),
                AAAN => PerGroup(Box::new(MilliGramm), Box::new(Gramm), NT),
               ... 
        }
    }
}
```

To use this generator create a `build.rs` with e.g.:

```
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("infood.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(
        &mut file,
        "{}",
        infood_tags_generator::generate()
    )
    .unwrap();
}
```

this will generate a `infood.rs` into your target.

This file you can then use to generate a module.

```
pub mod infood {
    //! Contains known infood tags generated from:
    //! https://www.fao.org/infoods/infoods/standards-guidelines/food-component-identifiers-tagnames/en/
    include!(concat!(env!("OUT_DIR"), "/infood.rs"));
}

```
