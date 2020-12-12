package days

import org.specs2.mutable.Specification

import scala.collection.mutable.ArrayBuffer

class day6Test extends Specification {
  val input: ArrayBuffer[ArrayBuffer[String]] =
    day6.loadInput("src/test/scala/days/day6_input.txt")
  val ex1: ArrayBuffer[ArrayBuffer[String]] = ArrayBuffer(
    ArrayBuffer("abc"),
    ArrayBuffer("a", "b", "c"),
    ArrayBuffer("ab", "ac"),
    ArrayBuffer("a", "a", "a", "a"),
    ArrayBuffer("b")
  )

  "part 1" should {
    day6.part1(ex1) === 11
    day6.part1(input) === 6662
  }

  "part 2" should {
    day6.part2(ex1) === 6
    day6.part2(input) === 3382
  }
}
