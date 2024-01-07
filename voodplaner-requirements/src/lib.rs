pub use voodplaner_core::Nutrient;
pub use voodplaner_core::{Age, Sex};

pub type KG = f32;
pub type CM = f32;
pub struct Persona {
    pub pal: pal::PAL,
    pub sex: Sex,
    pub age: Age,
    pub weight: KG,
    pub height: CM,
}

/// Contains default personas as per
/// https://www.dge.de/wissenschaft/referenzwerte/energie/#c1006
/// for PAL 1.4. This is convenient if you don't want to provide personal information but use a
/// default persona.
pub mod persona {
    use voodplaner_core::{Age, Sex};

    use crate::Persona;

    pub const MALE_0_MONTHS: Persona = Persona {
        pal: 1.0,
        sex: Sex::Male,
        age: Age::Months(0),
        weight: 5.6,
        height: 58.6,
    };
    pub const FEMALE_0_MONTHS: Persona = Persona {
        pal: 1.0,
        sex: Sex::Female,
        age: Age::Months(0),
        weight: 5.1,
        height: 57.1,
    };
    pub const MALE_4_MONTHS: Persona = Persona {
        pal: 1.0,
        sex: Sex::Male,
        age: Age::Months(4),
        weight: 8.6,
        height: 70.6,
    };
    pub const FEMALE_4_MONTHS: Persona = Persona {
        pal: 1.0,
        sex: Sex::Female,
        age: Age::Months(4),
        weight: 7.9,
        height: 68.7,
    };
    pub const MALE_1_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(1),
        weight: 13.9,
        height: 92.9,
    };
    pub const FEMALE_1_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(1),
        weight: 13.2,
        height: 91.3,
    };
    pub const MALE_4_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(4),
        weight: 20.2,
        height: 114.5,
    };
    pub const FEMALE_4_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(4),
        weight: 20.1,
        height: 114.3,
    };
    pub const MALE_7_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(7),
        weight: 29.3,
        height: 133.6,
    };
    pub const FEMALE_7_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(7),
        weight: 28.7,
        height: 132.4,
    };
    pub const MALE_10_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(10),
        weight: 41.0,
        height: 149.4,
    };
    pub const FEMALE_10_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(10),
        weight: 42.1,
        height: 151.0,
    };
    pub const MALE_13_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(13),
        weight: 55.5,
        height: 166.9,
    };
    pub const FEMALE_13_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(13),
        weight: 54.0,
        height: 162.7,
    };
    pub const MALE_15_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(15),
        weight: 69.2,
        height: 178.2,
    };
    pub const FEMALE_15_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(15),
        weight: 59.5,
        height: 165.5,
    };
    pub const MALE_19_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(19),
        weight: 70.8,
        height: 179.4,
    };
    pub const FEMALE_19_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(19),
        weight: 60.5,
        height: 165.8,
    };
    pub const MALE_25_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(25),
        weight: 70.7,
        height: 179.2,
    };
    pub const FEMALE_25_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(25),
        weight: 60.0,
        height: 165.1,
    };
    pub const MALE_51_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(51),
        weight: 68.7,
        height: 176.7,
    };
    pub const FEMALE_51_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(51),
        weight: 58.2,
        height: 162.6,
    };
    pub const MALE_65_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Male,
        age: Age::Years(65),
        weight: 66.8,
        height: 174.2,
    };
    pub const FEMALE_65_YEARS: Persona = Persona {
        pal: 1.4,
        sex: Sex::Female,
        age: Age::Years(65),
        weight: 57.1,
        height: 161.1,
    };
}

mod dge {
    include!(concat!(env!("OUT_DIR"), "/dge.rs"));
}
pub mod nutrients {
    use voodplaner_core::{Age, Nutrient, Sex};

    use crate::dge;

    pub fn lookup(sex: &Sex, age: &Age) -> &'static [Nutrient] {
        dge::lookup(sex, age)
    }
}
pub mod pal {

    pub type PAL = f32;

    pub enum ActivityLevel {
        Sleeping,
        Lying,
        Sedentary,
        Moderately,
        Very,
        Extremely,
    }

    impl ActivityLevel {
        pub fn to_pal(&self) -> PAL {
            match self {
                ActivityLevel::Sleeping => 0.95,
                ActivityLevel::Lying => 1.2,
                ActivityLevel::Sedentary => 1.4,
                ActivityLevel::Moderately => 1.6,
                ActivityLevel::Very => 1.8,
                ActivityLevel::Extremely => 2.0,
            }
        }
    }

    impl Into<PAL> for ActivityLevel {
        fn into(self) -> PAL {
            self.to_pal()
        }
    }

    pub struct ActivityPerHour {
        pub activity: ActivityLevel,
        pub hour: u8,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum CalculationError {
        IncorrectInput,
    }

    pub fn calculate_pal(activities: &[ActivityPerHour]) -> Result<PAL, CalculationError> {
        // sanity check for 24 hours
        if activities.iter().map(|x| x.hour).sum::<u8>() != 24 {
            return Err(CalculationError::IncorrectInput);
        }

        let sum: f32 = activities
            .iter()
            .map(|x| x.activity.to_pal() * x.hour as f32)
            .sum();
        Ok(sum / 24.0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn activity_level_to_pal() {
            assert_eq!(ActivityLevel::Sleeping.to_pal(), 0.95);
            assert_eq!(ActivityLevel::Lying.to_pal(), 1.2);
            assert_eq!(ActivityLevel::Sedentary.to_pal(), 1.4);
            assert_eq!(ActivityLevel::Moderately.to_pal(), 1.6);
            assert_eq!(ActivityLevel::Very.to_pal(), 1.8);
            assert_eq!(ActivityLevel::Extremely.to_pal(), 2.0);
        }

        #[test]
        fn pal_valid_input() {
            let activities = [
                ActivityPerHour {
                    activity: ActivityLevel::Sedentary,
                    hour: 8,
                },
                ActivityPerHour {
                    activity: ActivityLevel::Moderately,
                    hour: 8,
                },
                ActivityPerHour {
                    activity: ActivityLevel::Very,
                    hour: 8,
                },
            ];
            assert_eq!(calculate_pal(&activities).unwrap(), 1.6);
        }

        #[test]
        fn pal_insufficient_data() {
            let activities = [ActivityPerHour {
                activity: ActivityLevel::Sedentary,
                hour: 8,
            }];
            assert_eq!(
                calculate_pal(&activities),
                Err(CalculationError::IncorrectInput)
            );
        }

        #[test]
        fn pal_excessive_data() {
            let activities = [
                ActivityPerHour {
                    activity: ActivityLevel::Sedentary,
                    hour: 12,
                },
                ActivityPerHour {
                    activity: ActivityLevel::Very,
                    hour: 13,
                },
            ];
            assert_eq!(
                calculate_pal(&activities),
                Err(CalculationError::IncorrectInput)
            );
        }
    }
}

pub mod energy {
    use voodplaner_core::Sex;

    pub type Energy = f32;
    pub type Months = u16;

    const MALE_LOOKUP: [(Months, u16, Energy); 12] = [
        (0, 4, 1.66),
        (4, 12, 1.15),
        (12, 48, 0.66),
        (48, 84, 0.44),
        (84, 120, 0.31),
        (120, 156, 0.22),
        (156, 180, 0.18),
        (180, 228, 0.15),
        (228, 300, 0.14),
        (300, 612, 0.13),
        (612, 780, 0.14),
        (780, 2999, 0.13),
    ];
    const FEMALE_LOOKUP: [(Months, u16, Energy); 12] = [
        (0, 4, 1.71),
        (4, 12, 1.15),
        (12, 48, 0.65),
        (48, 84, 0.43),
        (84, 120, 0.31),
        (120, 156, 0.21),
        (156, 180, 0.16),
        (180, 228, 0.14),
        (228, 300, 0.14),
        (300, 612, 0.13),
        (612, 780, 0.14),
        (780, 2999, 0.14),
    ];

    pub fn calculate(persona: &crate::Persona) -> Energy {
        let lookup = match persona.sex {
            Sex::Male => MALE_LOOKUP,
            Sex::Female => FEMALE_LOOKUP,
        };
        let months = persona.age.months();
        let base_energy = lookup
            .iter()
            // Differing from DGE, ages are treated non-inclusively.
            .find(|x| x.0 >= months)
            .map(|x| x.2)
            .unwrap_or(0.14);

        base_energy * persona.weight * persona.height * persona.pal
    }

    #[cfg(test)]
    mod tests {

        macro_rules! generate_tests {
            ($($persona:ident, $expected_pal:expr);*) => {
                $(
                    #[test]
                    #[allow(non_snake_case)]
                    fn $persona() {
                        let result = crate::energy::calculate(&crate::persona::$persona);
                        assert_eq!(result, $expected_pal);
                    }
                )*
            };
        }

        generate_tests! {
            MALE_0_MONTHS,544.74554;
            FEMALE_0_MONTHS,497.9691;
            MALE_4_MONTHS,698.234;
            FEMALE_4_MONTHS,624.13947;
            MALE_1_YEARS,1193.1704;
            FEMALE_1_YEARS,1096.6956;
            MALE_4_YEARS,1424.7465;
            FEMALE_4_YEARS,1383.053;
            MALE_7_YEARS,1698.8844;
            FEMALE_7_YEARS,1649.1478;
            MALE_10_YEARS,1886.623;
            FEMALE_10_YEARS,1868.9873;
            MALE_13_YEARS,2334.2634;
            FEMALE_13_YEARS,1968.019;
            MALE_15_YEARS,2589.6023;
            FEMALE_15_YEARS,1930.0609;
            MALE_19_YEARS,2489.498;
            FEMALE_19_YEARS,1966.0564;
            MALE_25_YEARS,2305.8376;
            FEMALE_25_YEARS,1802.892;
            MALE_51_YEARS,2379.3008;
            FEMALE_51_YEARS,1854.8108;
            MALE_65_YEARS,2117.854;
            FEMALE_65_YEARS,1802.9669
        }
    }
}

pub mod recommendation {
    use voodplaner_core::Nutrient;

    use crate::Persona;

    pub fn daily<'a, T>(personas: T) -> Vec<Vec<Nutrient>>
    where
        T: IntoIterator<Item = &'a Persona>,
    {
        fn calculate(persona: &Persona) -> Vec<Nutrient> {
            let nutrients = crate::nutrients::lookup(&persona.sex, &persona.age);
            let mut result = Vec::with_capacity(nutrients.len() - 5);
            let mut energy_parts = Vec::with_capacity(6);
            let base_energy = crate::energy::calculate(persona);
            use voodplaner_core::Amount::*;
            for n in nutrients {
                match n.amount() {
                    PercentageOfEnergy(x) => {
                        let value = KCal(base_energy * x);
                        energy_parts.push(n.with_amount(value));
                    }
                    GrammPerWeight(x) => {
                        let value = GrammPerDay(x * persona.weight);
                        result.push(n.with_amount(value));
                    }
                    KCal(_) => energy_parts.push(n.clone()), // this should actually not happen?
                    GrammPerDay(_) | MilliGramPerDay(_) | MicroGramPerDay(_) => {
                        result.push(n.clone())
                    }
                }
            }

            result.push(crate::Nutrient::Energy(
                voodplaner_core::Amount::KCal(base_energy),
                energy_parts,
            ));
            result
        }
        personas.into_iter().map(calculate).collect()
    }

    #[cfg(test)]
    mod tests {
        use voodplaner_core::Nutrient;

        use crate::persona;

        #[test]
        pub fn energy_mix() {
            let examples = [
                persona::MALE_0_MONTHS,
                persona::FEMALE_0_MONTHS,
                persona::MALE_4_MONTHS,
                persona::FEMALE_4_MONTHS,
                persona::MALE_1_YEARS,
                persona::FEMALE_1_YEARS,
                persona::MALE_4_YEARS,
                persona::FEMALE_4_YEARS,
                persona::MALE_7_YEARS,
                persona::FEMALE_7_YEARS,
                persona::MALE_10_YEARS,
                persona::FEMALE_10_YEARS,
                persona::MALE_13_YEARS,
                persona::FEMALE_13_YEARS,
                persona::MALE_15_YEARS,
                persona::FEMALE_15_YEARS,
                persona::MALE_19_YEARS,
                persona::FEMALE_19_YEARS,
                persona::MALE_25_YEARS,
                persona::FEMALE_25_YEARS,
                persona::MALE_51_YEARS,
                persona::FEMALE_51_YEARS,
                persona::MALE_65_YEARS,
                persona::FEMALE_65_YEARS,
            ];

            let recommentation = crate::recommendation::daily(&examples);
            let energy_sum: f32 = recommentation
                .iter()
                .flat_map(|x| {
                    x.iter().filter_map(|x| match x {
                        Nutrient::Energy(a, _) => match a {
                            voodplaner_core::Amount::KCal(v) => Some(v),
                            _ => None,
                        },
                        _ => None,
                    })
                })
                .sum();

            let energy_parts_sum: f32 = recommentation
                .iter()
                .flat_map(|x| {
                    x.iter().filter(|x| x.healthy()).filter_map(|x| match x {
                        Nutrient::Energy(_, x) => Some(
                            x.iter()
                                .filter_map(|x| match x.amount() {
                                    voodplaner_core::Amount::KCal(v) => Some(v),
                                    _ => None,
                                })
                                .sum::<f32>(),
                        ),

                        _ => None,
                    })
                })
                .sum();
            
            assert!(
                energy_parts_sum > energy_sum,
                "A lot of the percentage is an up to value, this is not reflected in code"
            );
        }
    }
}
