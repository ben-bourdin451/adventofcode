package days

import org.specs2.mutable.Specification

class day7Test extends Specification {
  val input: Seq[String] =
    fileUtils.loadFile("src/test/scala/days/day7_input.txt")
  val ex1 = List(
    "light red bags contain 1 bright white bag, 2 muted yellow bags.",
    "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
    "bright white bags contain 1 shiny gold bag.",
    "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
    "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
    "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
    "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
    "faded blue bags contain no other bags.",
    "dotted black bags contain no other bags."
  )

  val ex2 = List(
    "shiny gold bags contain 2 dark red bags.",
    "dark red bags contain 2 dark orange bags.",
    "dark orange bags contain 2 dark yellow bags.",
    "dark yellow bags contain 2 dark green bags.",
    "dark green bags contain 2 dark blue bags.",
    "dark blue bags contain 2 dark violet bags.",
    "dark violet bags contain no other bags.",
  )

  "part 1" should {
    day7.part1(ex1) === 4
    day7.part1(input) === 252
  }

  "part 2" should {
    day7.part2(ex1) === 32
    day7.part2(ex2) === 126
    day7.part2(input) === 35487
  }
}
