use crate::lexer::Lexer;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;
use crate::tests::read_from_file;

mod cursor_test;
mod expr_tests;
mod item_tests;
mod file_tests;
mod stmt_tests;

fn get_parser(input: &str) -> ParseCursor {
    let mut lexer = Lexer::new(input);
    ParseCursor::new(lexer.tokenize())
}

fn parse_input<T: Parse>(input: &str) -> Result<T, RccError> {
    let mut lexer = Lexer::new(input);
    let mut cxt = ParseCursor::new(lexer.tokenize());
    T::parse(&mut cxt)
}

fn parse_validate<T: Parse>(
    inputs: std::vec::Vec<&str>,
    excepteds: Vec<Result<T, RccError>>,
) {
    assert_eq!(inputs.len(), excepteds.len());
    for (input, excepted) in inputs.into_iter().zip(excepteds) {
        let result = parse_input::<T>(input);
        match excepted {
            Ok(segments) => assert_eq!(Ok(segments), result),
            Err(s) => {
                assert_eq!(result.unwrap_err(), s)
            },
        }
    }
}

fn expected_from_file(file_name: &str) -> String {
    read_from_file(file_name, "./src/parser/tests")
}
