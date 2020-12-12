package days

import days.day8.extractValue
import org.specs2.mutable.Specification

class day8Test extends Specification {
  val input: Seq[String] =
    fileUtils.loadFile("src/test/scala/days/day8_input.txt")
  val ex1 = List(
    "nop +0",
    "acc +1",
    "jmp +4",
    "acc +3",
    "jmp -3",
    "acc -99",
    "acc +1",
    "jmp -4",
    "acc +6",
  )

  "part 1" should {
    day8.part1(ex1) === 5
    day8.part1(input) === 1446
  }

  "part 2" should {
    day8.part2(ex1) === 8
    day8.part2(input) === 1403
  }
}
