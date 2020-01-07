use super::common::{self, input_list};

/// Calculates the basic fuel requirements of a module with a given mass.
///
/// This does not take into account the fuel requirements of the fuel added itself, referred to
/// as the complete fuel requirement.
///
/// # Examples
///
/// ```
/// use advent_of_code_2019::day1;
///
/// assert_eq!(day1::calculate_basic_fuel_req_for_module(&12), 2);
/// assert_eq!(day1::calculate_basic_fuel_req_for_module(&14), 2);
/// assert_eq!(day1::calculate_basic_fuel_req_for_module(&1969), 654);
/// assert_eq!(day1::calculate_basic_fuel_req_for_module(&100756), 33583);
/// ```
pub fn calculate_basic_fuel_req_for_module(module_mass: &i32) -> i32 {
    ((module_mass / 3) - 2).max(0)
}

/// Calculates the total basic fuel requirements of a set of modules given their masses.
///
/// This does not take into account the fuel requirements of the fuel added itself, referred to
/// as the complete fuel requirement.
///
/// # Examples
///
/// ```
/// use advent_of_code_2019::day1;
///
/// assert_eq!(day1::calculate_basic_fuel_req_for_modules(&vec![12]), 2);
/// assert_eq!(day1::calculate_basic_fuel_req_for_modules(&vec![12, 14]), 4);
/// assert_eq!(day1::calculate_basic_fuel_req_for_modules(&vec![12, 14, 1969]), 658);
/// assert_eq!(day1::calculate_basic_fuel_req_for_modules(&vec![12, 14, 1969, 100756]), 34_241);
/// ```
pub fn calculate_basic_fuel_req_for_modules(module_masses: &[i32]) -> i32 {
    module_masses
        .iter()
        .map(calculate_basic_fuel_req_for_module)
        .sum()
}

pub struct Day1Runner {}

impl common::DayRunner for Day1Runner {
    fn run(&self, args: &Vec<String>) -> common::Result {
        if args.len() < 1 {
            return Err(common::Error::from(
                "Please provide the name of the file to load input from relative to the \
                 current working directory.",
            ));
        } else if args.len() > 1 {
            return Err(common::Error::from(
                "Please provide only the name of the file to load input from relative to the \
                 current working directory.",
            ));
        }

        let masses: Vec<i32> = input_list::InputList::new_from_file(&args[0])?.parse()?;
        let fuel_requirement = calculate_basic_fuel_req_for_modules(&masses[..]);

        println!(
            "Basic fuel requirements for all {} modules: {}",
            masses.len(),
            fuel_requirement
        );

        Ok(())
    }
}
