pub use voodplaner_core::Nutrient;
pub use voodplaner_core::{Age, Recommandation, Sex};

mod dge {
    include!(concat!(env!("OUT_DIR"), "/dge.rs"));
}
pub mod nutrients {
    use voodplaner_core::{Age, Nutrient, Sex};

    use crate::dge;

    pub fn lookup(sex: Sex, age: Age) -> &'static [Nutrient] {
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

    pub enum CalculationError {
        InsufficientData,
    }

    pub fn calculate_pal(activities: &[ActivityPerHour]) -> Result<PAL, CalculationError> {
        // sanity check for 24 hours
        if activities.iter().map(|x| x.hour).sum::<u8>() != 24 {
            return Err(CalculationError::InsufficientData);
        }

        let sum: f32 = activities
            .iter()
            .map(|x| x.activity.to_pal() * x.hour as f32)
            .sum();
        Ok(sum / 24.0)
    }
}

pub mod energy {
    use voodplaner_core::{Age, Sex};

    use crate::pal;

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
    type KG = f32;
    type CM = f32;

    pub fn calculate_energy(pal: pal::PAL, sex: Sex, age: Age, weight: KG, height: CM) -> Energy {
        let lookup = match sex {
            Sex::Male => MALE_LOOKUP,
            Sex::Female => FEMALE_LOOKUP,
        };
        let months = age.months();
        let base_energy = lookup
            .iter()
            .find(|x| x.0 >= months)
            .map(|x| x.2)
            .unwrap_or(0.14);
        base_energy * weight * height * pal
    }
}
