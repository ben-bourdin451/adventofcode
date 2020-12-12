package days

object day3 {
  def part1(input: Seq[String]): Int = {
    slope(input, 3, 1)
  }

  def slope(input: Seq[String], right: Int, down: Int): Int = {
    var x: Int = 0
    var y: Int = 0

    var trees = 0
    val len = input.head.length()
    while (y < input.length) {
      if (input(y)(x % len) == '#') {
        trees += 1
      }
      x += right
      y += down
    }
    trees
  }

  def part2(input: Seq[String]): BigInt = {
    val l = List(
      slope(input, 1, 1),
      slope(input, 3, 1),
      slope(input, 5, 1),
      slope(input, 7, 1),
      slope(input, 1, 2)
    )
    var p = BigInt(1)
    l.foreach(p *= _)
    p
  }
}
