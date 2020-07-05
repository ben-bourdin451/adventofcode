object Main {
    def main(args: Array[String]) = {
      val me = Person(Point(0,0), Up)
      val input = "R4, R3, R5, L3, L5, R2, L2, R5, L2, R5, R5, R5, R1, R3, L2, L2, L1, R5, L3, R1, L2, R1, L3, L5, L1, R3, L4, R2, R4, L3, L1, R4, L4, R3, L5, L3, R188, R4, L1, R48, L5, R4, R71, R3, L2, R188, L3, R2, L3, R3, L5, L1, R1, L2, L4, L2, R5, L3, R3, R3, R4, L3, L4, R5, L4, L4, R3, R4, L4, R1, L3, L1, L1, R4, R1, L4, R1, L1, L3, R2, L2, R2, L1, R5, R3, R4, L5, R2, R5, L5, R1, R2, L1, L3, R3, R1, R3, L4, R4, L4, L1, R1, L2, L2, L4, R1, L3, R4, L2, R3, L1, L5, R4, R5, R2, R5, R1, R5, R1, R3, L3, L2, L2, L5, R2, L2, R5, R5, L2, R3, L5, R5, L2, R4, R2, L1, R3, L5, R3, R2, R5, L1, R3, L2, R2, R1"
      val target = Instruction.fromMultiple(input).foldLeft(me){
        case (person, instruction: Instruction) => person.walk(instruction)
      }

      println(s"day 1 part 1: ${target.p.distance(me.p)}")
    }
}

case class Point(x: Int, y: Int) {
  def stepInDirection(instruction: Instruction): Point = {
    instruction.direction match {
      case Left => copy(x = x-instruction.steps)
      case Right => copy(x = x+instruction.steps)
      case Down => copy(y = y+instruction.steps)
      case Up => copy(y = y-instruction.steps)
    }
  }

  def distance(p: Point): Int = {
    return Math.abs(this.x - p.x) + Math.abs(this.y - p.y)
  }
}
trait Direction {
  val toLeft:Direction
  val toRight:Direction
}
case object Left extends Direction {
  override val toLeft: Direction = Down
  override val toRight: Direction = Up
}
case object Right extends Direction {
  override val toLeft: Direction = Up
  override val toRight: Direction = Down
}
case object Up extends Direction {
  override val toLeft: Direction = Left
  override val toRight: Direction = Right
}
case object Down extends Direction {
  override val toLeft: Direction = Right
  override val toRight: Direction = Left
}
object Direction {
  def apply(in: String): Direction = {
    in match {
      case "L" => Left
      case "R" => Right
      case "U" => Right
      case "D" => Right
    }
  }
}
case class Instruction(direction: Direction, steps: Int)
object Instruction {
  def apply(input: String):Instruction = {
    val split = input.split("")
    Instruction(Direction(split(0)), split(1).toInt)
  }
  def fromMultiple(list: String):List[Instruction] = {
    list.split(",").map(in => Instruction(in.trim())).toList
  }
}
case class Person(p: Point, orientation: Direction) {
  def walk(instruction: Instruction):Person = {
    val newOrientation = instruction.direction match {
      case Left => orientation.toLeft
      case Right => orientation.toRight
    }
    val newPoint = p.stepInDirection(Instruction(newOrientation, instruction.steps))
    copy(newPoint, newOrientation)
  }
}
