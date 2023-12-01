import gleeunit
import gleeunit/should
import day01

pub fn main() {
  gleeunit.main()
}

pub fn part1_test() {
  day01.part1() |> should.equal("55002")
}

pub fn part2_test() {
  day01.part2() |> should.equal("55093")
}
