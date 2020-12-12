package days

import org.specs2.mutable.Specification

class day9Test extends Specification {
  val input: Seq[BigInt] =
    fileUtils
      .loadFile("src/test/scala/days/day9_input.txt")
      .map(BigInt(_))
  val ex1: Seq[BigInt] = List(
    "35",
    "20",
    "15",
    "25",
    "47",
    "40",
    "62",
    "55",
    "65",
    "95",
    "102",
    "117",
    "150",
    "182",
    "127",
    "219",
    "299",
    "277",
    "309",
    "576"
  ).map(BigInt(_))

  "hasSum" should {
    day9.hasSum(List(1, 2, 3, 4, 25, 5, 16), 26) === true
    day9.hasSum(List(1, 2, 3, 4, 24, 5, 16), 26) === true
    day9.hasSum(List(1, 2, 3, 4, 20, 5, 16), 26) === false
    day9.hasSum(List(95, 102, 117, 150, 182), 127) === false
    day9.hasSum(List(65, 95, 102, 117, 150), 182) === true
  }

  "part 1" should {
    day9.part1(ex1, 5) === BigInt(127)
    day9.part1(input, 25) === BigInt(105950735)
  }

  "part 2" should {
    day9.part2(ex1, 127) === BigInt(62)
    day9.part2(input, 105950735) === BigInt(13826915)
  }
}
