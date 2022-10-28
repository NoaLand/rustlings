// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// --------------------------------------------------
// solution 1
//
// use std::borrow::Borrow;
// use std::fmt;
// use std::fmt::{Formatter, write};
//
// pub enum GradeFormat {
//     Numerical(f32),
//     Alphabetical(String),
// }
//
// impl fmt::Display for GradeFormat {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             GradeFormat::Numerical(grade) => write!(f, "{}", grade),
//             GradeFormat::Alphabetical(grade) => write!(f, "{}", grade),
//         }
//     }
// }
//
// impl From<f32> for GradeFormat {
//     fn from(numerical_grade: f32) -> Self {
//         Self::Numerical(numerical_grade)
//     }
// }
//
// impl From<String> for GradeFormat {
//     fn from(alphabetical_grade: String) -> Self {
//         Self::Alphabetical(alphabetical_grade)
//     }
// }
//
// mod sealed {
//     pub trait Sealed {}
//     impl Sealed for f32 {}
//     impl Sealed for String {}
// }
//
// pub struct ReportCard<T: Into<GradeFormat> + sealed::Sealed + Clone> {
//     pub grade: T,
//     pub student_name: String,
//     pub student_age: u8,
// }
//
// impl<T: Into<GradeFormat> + sealed::Sealed + Clone> ReportCard<T> {
//     pub fn print(&self) -> String {
//         format!("{} ({}) - achieved a grade of {}",
//             &self.student_name, &self.student_age, &self.grade.clone().into())
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn generate_numeric_report_card() {
//         let report_card = ReportCard {
//             grade: 2.1,
//             student_name: "Tom Wriggle".to_string(),
//             student_age: 12,
//         };
//         assert_eq!(
//             report_card.print(),
//             "Tom Wriggle (12) - achieved a grade of 2.1"
//         );
//     }
//
//     #[test]
//     fn generate_alphabetic_report_card() {
//         // TODO: Make sure to change the grade here after you finish the exercise.
//         let report_card = ReportCard {
//             grade: "A+".to_string(),
//             student_name: "Gary Plotter".to_string(),
//             student_age: 11,
//         };
//         assert_eq!(
//             report_card.print(),
//             "Gary Plotter (11) - achieved a grade of A+"
//         );
//     }
// }
// --------------------------------------------------

use std::borrow::Borrow;
use std::fmt;
use std::fmt::{Display, Formatter, write};

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
