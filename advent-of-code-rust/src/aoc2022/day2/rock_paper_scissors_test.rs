#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};
    use crate::prelude::*;
    use super::super::rock_paper_scissors::*;

    const PART_ONE_ANSWER: &'static str = "12458";
    const PART_TWO_ANSWER: &'static str = "12683";

    #[fixture]
    fn implementation() -> impl solution::Solution {
        RockPaperScissors {}
    }

    #[fixture]
    fn sample_input_content() -> String {
        utils::get_labelled_input(file!(), "-sample")
    }

    #[rstest]
    fn test_solve(implementation: impl solution::Solution) {
        solution_test::solve_all(file!(), implementation);
    }

    #[rstest]
    fn test_solve_part_one(implementation: impl solution::Solution, sample_input_content: String) {
        assert_eq!(implementation.solve_part_one(&sample_input_content), PART_ONE_ANSWER)
    }

    #[rstest]
    fn test_solve_part_two(implementation: impl solution::Solution, sample_input_content: String) {
        assert_eq!(implementation.solve_part_two(&sample_input_content), PART_TWO_ANSWER)
    }

    #[rstest]
    fn test_solve_single_round_win(implementation: impl solution::Solution) {
        let input = "A Y".to_string();
        assert_eq!(implementation.solve_part_one(&input), "8")
    }
}
