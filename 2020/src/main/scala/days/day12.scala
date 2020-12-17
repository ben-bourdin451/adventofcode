package days

case class Point(x: Int, y: Int) {}
object day12 {
  val directions: Map[Int, Point] = Map[Int, Point](
    0 -> Point(1, 0),
    90 -> Point(0, 1),
    180 -> Point(-1, 0),
    270 -> Point(0, -1)
  )

  def rotateWaypointLeft(waypoint: Point, degrees: Int): Point = {
    degrees match {
      case 90  => Point(-waypoint.y, waypoint.x)
      case 180 => Point(-waypoint.x, -waypoint.y)
      case 270 => rotateWaypointRight(waypoint, 90)
      case _   => waypoint
    }
  }

  def rotateWaypointRight(waypoint: Point, degrees: Int): Point = {
    degrees match {
      case 90  => Point(waypoint.y, -waypoint.x)
      case 180 => Point(-waypoint.x, -waypoint.y)
      case 270 => rotateWaypointLeft(waypoint, 90)
      case _   => waypoint
    }
  }

  def part1(in: Seq[String]): Int = {
    var d = 0
    var pos = Point(0, 0)
    in.foreach(inst => {
      val n: Int = Integer.valueOf(inst.slice(1, inst.length)).toInt
      inst match {
        case _ if inst(0) == 'N' => pos = Point(pos.x, pos.y + n)
        case _ if inst(0) == 'S' => pos = Point(pos.x, pos.y - n)
        case _ if inst(0) == 'E' => pos = Point(pos.x + n, pos.y)
        case _ if inst(0) == 'W' => pos = Point(pos.x - n, pos.y)
        case _ if inst(0) == 'L' => d = (d + n) % 360
        case _ if inst(0) == 'R' => d = (360 + d - n) % 360
        case _ if inst(0) == 'F' =>
          pos = Point(pos.x + directions(d).x * n, pos.y + directions(d).y * n)
      }
    })
    math.abs(pos.x) + math.abs(pos.y)
  }

  def part2(in: Seq[String]): Int = {
    var pos = Point(0, 0)
    var waypoint = Point(10, 1)
    in.foreach(inst => {
      val n: Int = Integer.valueOf(inst.slice(1, inst.length)).toInt
      inst match {
        case _ if inst(0) == 'N' => waypoint = Point(waypoint.x, waypoint.y + n)
        case _ if inst(0) == 'S' => waypoint = Point(waypoint.x, waypoint.y - n)
        case _ if inst(0) == 'E' => waypoint = Point(waypoint.x + n, waypoint.y)
        case _ if inst(0) == 'W' => waypoint = Point(waypoint.x - n, waypoint.y)
        case _ if inst(0) == 'L' => waypoint = rotateWaypointLeft(waypoint, n)
        case _ if inst(0) == 'R' => waypoint = rotateWaypointRight(waypoint, n)
        case _ if inst(0) == 'F' =>
          pos = Point(pos.x + waypoint.x * n, pos.y + waypoint.y * n)
      }
    })
    math.abs(pos.x) + math.abs(pos.y)
  }
}
