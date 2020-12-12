package days

import org.specs2.mutable.Specification

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

class day10Test extends Specification {
  val input: Seq[Int] =
    fileUtils
      .loadFile("src/test/scala/days/day10_input.txt")
      .map(Integer.valueOf(_).toInt)

  val ex1: Seq[Int] =
    List[Int](16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4)
  val ex2: Seq[Int] =
    List[Int](28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38,
      39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3)

  "part 1" should {
    day10.part1(ex1) === 35 // 7 * 5
    day10.part1(ex2) === 220 // 22 * 10
    day10.part1(input) === 1914 // 66 * 29
  }

  "permutations" should {
    day10.fib3(0) === 1
    day10.fib3(1) === 1
    day10.fib3(2) === 2
    day10.fib3(3) === 4
    day10.fib3(4) === 7
    day10.fib3(5) === 13
  }

  "part 2" should {
    day10.part2(ex1) === BigInt(8)
    day10.part2(ex2) === BigInt(19208)
    day10.part2(input) === BigInt("9256148959232")
  }
}
