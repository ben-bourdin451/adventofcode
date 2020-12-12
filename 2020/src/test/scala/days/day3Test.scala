package days

import org.specs2.mutable.Specification

class day3Test extends Specification {
  val input: Seq[String] =
    fileUtils.loadFile("src/test/scala/days/day3_input.txt")
  val ex1 = List(
    "..##.......",
    "#...#...#..",
    ".#....#..#.",
    "..#.#...#.#",
    ".#...##..#.",
    "..#.##.....",
    ".#.#.#....#",
    ".#........#",
    "#.##...#...",
    "#...##....#",
    ".#..#...#.#"
  )

  "part 1" should {
    day3.part1(ex1) === 7
    day3.part1(input) === 228
  }

  "part 2" should {
    day3.part2(ex1) === BigInt(336)
    day3.part2(input) === BigInt("6818112000")
  }
}
