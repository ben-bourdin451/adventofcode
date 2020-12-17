package days

import org.specs2.mutable.Specification

class day12Test extends Specification {
  val input: Seq[String] =
    fileUtils
      .loadFile("src/test/scala/days/day12_input.txt")

  val ex1 = List("F10", "N3", "F7", "R90", "F11")

  "rotate waypoint" should {
    day12.rotateWaypointRight(Point(10, 4), 90) === Point(4, -10)
    day12.rotateWaypointRight(Point(-4, 10), 90) === Point(10, 4)
    day12.rotateWaypointLeft(Point(10, 4), 90) === Point(-4, 10)
    day12.rotateWaypointLeft(Point(-4, 10), 90) === Point(-10, -4)

    day12.rotateWaypointRight(Point(10, 4), 90) === Point(4, -10)
    day12.rotateWaypointRight(Point(-4, 10), 90) === Point(10, 4)
    day12.rotateWaypointLeft(Point(10, 4), 90) === Point(-4, 10)
    day12.rotateWaypointLeft(Point(-4, 10), 90) === Point(-10, -4)
  }

  "part 1" should {
    day12.part1(ex1) === 25
    day12.part1(input) === 1424
  }

  "part 2" should {
    day12.part2(ex1) === 286
    day12.part2(input) === 63447
  }
}
