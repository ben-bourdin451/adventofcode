package days

import org.specs2.mutable.Specification

class day5Test extends Specification {
  val input: Seq[String] =
    fileUtils.loadFile("src/test/scala/days/day5_input.txt")
  val ex1: Seq[String] = List()

  "part 1" should {
    day5.part1(List("BFFFBBFRRR")) === 567
    day5.part1(List("FFFBBBFRRR")) === 119
    day5.part1(List("BBFFBBFRLL")) === 820
    day5.part1(List("BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL")) === 820
    day5.part1(input) === 878
  }

  "part 2" should {
    day5.part2(input) === 504
  }
}
