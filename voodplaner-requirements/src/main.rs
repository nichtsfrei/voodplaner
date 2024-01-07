use voodplaner_core::Nutrient;

fn main() {
    use voodplaner_requirements::persona;

    let examples = [
        (persona::MALE_0_MONTHS, "Anton", 0.0),
        (persona::FEMALE_0_MONTHS, "Anna", 0.0),
        (persona::MALE_4_MONTHS, "Adrian", 0.33),
        (persona::FEMALE_4_MONTHS, "Amelia", 0.33),
        (persona::MALE_1_YEARS, "Alexander", 1.0),
        (persona::FEMALE_1_YEARS, "Alicia", 1.0),
        (persona::MALE_4_YEARS, "Arthur", 4.0),
        (persona::FEMALE_4_YEARS, "Ava", 4.0),
        (persona::MALE_7_YEARS, "August", 7.0),
        (persona::FEMALE_7_YEARS, "Aurora", 7.0),
        (persona::MALE_10_YEARS, "Axel", 10.0),
        (persona::FEMALE_10_YEARS, "Alexandra", 10.0),
        (persona::MALE_13_YEARS, "Aaron", 13.0),
        (persona::FEMALE_13_YEARS, "Anastasia", 13.0),
        (persona::MALE_15_YEARS, "Andreas", 15.0),
        (persona::FEMALE_15_YEARS, "Anja", 15.0),
        (persona::MALE_19_YEARS, "Albert", 19.0),
        (persona::FEMALE_19_YEARS, "Alina", 19.0),
        (persona::MALE_25_YEARS, "Andre", 25.0),
        (persona::FEMALE_25_YEARS, "Anke", 25.0),
        (persona::MALE_51_YEARS, "Armin", 51.0),
        (persona::FEMALE_51_YEARS, "Astrid", 51.0),
        (persona::MALE_65_YEARS, "Alfred", 65.0),
        (persona::FEMALE_65_YEARS, "Agnes", 65.0),
    ];

    let recommentation =
        voodplaner_requirements::recommendation::daily(examples.iter().map(|x| &x.0));
    println!("# Example Daily Recommendation");
    println!("This is an exmple of a daily recommendation of a tribe.");
    println!("This tribe does light computer work for 23h. They eat their daily food portion in one sitting in a hour.");
    println!();
    println!();
    println!("They consist of the members:");
    for x in &examples {
        println!("- {} {} years old", x.1, x.2)
    }
    println!();
    println!();
    println!("Which conveniently is exactly setup in our default personas.");
    for (i, x) in examples.iter().enumerate() {
        println!("## Recommendation for {}", x.1);
        println!("| Name | Amount |");
        println!("| ---- | ------ |");
        for y in &recommentation[i] {
            println!("| {} | {} |", y.name(), y.amount());
            // syould be the last parameter
            if let Nutrient::Energy(_, x) = y {
                println!("### Energy mix");
                println!("| Name | Amount |");
                println!("| ---- | ------ |");
                for z in x {
                    println!("| {} | {} |", z.name(), z.amount());
                }
            }
        }
    }
}
