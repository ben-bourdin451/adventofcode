package days

import org.specs2.mutable.Specification

class day13Test extends Specification {
  val input: Seq[String] =
    fileUtils
      .loadFile("src/test/scala/days/day13_input.txt")

  "part 1" should {
    day13.part1(
      939,
      "7,13,x,x,59,x,31,19"
        .split(",")
        .filter(_ != "x")
        .map(Integer.valueOf(_).toInt)
        .toSeq
    ) === 295

    day13.part1(
      Integer.valueOf(input.head).toInt,
      input.last
        .split(",")
        .filter(_ != "x")
        .map(Integer.valueOf(_).toInt)
        .toSeq
    ) === 2545

  }

  "part 2" should {
    day13.part2("17,x,13,19".split(",")) === 3417
    day13.part2("7,13,x,x,59,x,31,19".split(",")) === 1068781
    day13.part2("67,7,59,61".split(",")) === 754018
    day13.part2("67,x,7,59,61".split(",")) === 779210
    day13.part2("67,7,x,59,61".split(",")) === 1261476
    day13.part2("1789,37,47,1889".split(",")) === 1202161486
    day13.part2(input.last.split(",")) === BigInt("266204454441577")
  }
}
