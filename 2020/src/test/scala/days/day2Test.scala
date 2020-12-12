package days

import org.specs2.mutable.Specification

class day2Test extends Specification {
  val input: Seq[String] =
    fileUtils.loadFile("src/test/scala/days/day2_input.txt")
  val ex1 = List("1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc")

  "part 1" should {
    day2.solve(ex1, day2.part1) === 2
    day2.solve(input, day2.part1) === 538
  }

  "part 2" should {
    day2.solve(ex1, day2.part2) === 1
    day2.solve(input, day2.part2) === 489
  }
}
