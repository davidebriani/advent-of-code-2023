import gleam/io
import gleam/int
import gleam/list
import gleam/option
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  part1() |> io.println()
  part2() |> io.println()
}

pub fn part2() {
  solve_puzzle(parse_digit)
}

pub fn part1() {
  solve_puzzle(parse_numeric_digit)
}

type DigitParser = fn (String) -> option.Option(Int)

fn solve_puzzle(digit_parser: DigitParser) {
  simplifile.read("input.txt")
  |> result.unwrap("")
  |> string.split("\n")
  |> list.map(get_calibration_number(_, digit_parser))
  |> result.all()
  |> result.unwrap([])
  |> int.sum()
  |> int.to_string()
}

fn get_calibration_number(line: String, digit_parser: DigitParser) {
  let first_digit = get_first_digit(line, digit_parser, 0)
  let last_digit = get_last_digit(line, digit_parser, 0)
  [first_digit, last_digit] |> int.undigits(10)
}

fn get_first_digit(text: String, digit_parser: DigitParser, drop: Int) {
  case digit_parser(string.slice(text, at_index: drop, length: string.length(text))) {
    option.Some(digit) -> digit
    option.None -> get_first_digit(text, digit_parser, drop + 1)
  }
}

fn get_last_digit(text: String, digit_parser: DigitParser, drop: Int) {
  case digit_parser(string.slice(text, at_index: -drop - 1, length: string.length(text))) {
    option.Some(digit) -> digit
    option.None -> get_last_digit(text, digit_parser, drop + 1)
  }
}

fn parse_digit(text: String) {
  option.or(parse_literal_digit(text), parse_numeric_digit(text))
}

fn parse_literal_digit(text: String) {
  case text {
    "one" <> _ -> option.Some(1)
    "two" <> _ -> option.Some(2)
    "three" <> _ -> option.Some(3)
    "four" <> _ -> option.Some(4)
    "five" <> _ -> option.Some(5)
    "six" <> _ -> option.Some(6)
    "seven" <> _ -> option.Some(7)
    "eight" <> _ -> option.Some(8)
    "nine" <> _ -> option.Some(9)
    _other -> option.None
  }
}

fn parse_numeric_digit(text: String) {
  case text {
    "1" <> _ -> option.Some(1)
    "2" <> _ -> option.Some(2)
    "3" <> _ -> option.Some(3)
    "4" <> _ -> option.Some(4)
    "5" <> _ -> option.Some(5)
    "6" <> _ -> option.Some(6)
    "7" <> _ -> option.Some(7)
    "8" <> _ -> option.Some(8)
    "9" <> _ -> option.Some(9)
    _other -> option.None
  }
}
