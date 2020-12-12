package days

object day5 {
  def toBinary(in: String): Int = {
    val binary = in
      .replace('F', '0')
      .replace('B', '1')
      .replace('L', '0')
      .replace('R', '1')
    Integer.parseInt(binary, 2)
  }

  def part1(input: Seq[String]): Int = {
    input.map(toBinary).max
  }

  def part2(input: Seq[String]): Int = {
    val seats = input.map(toBinary)
    (((input.length + 1) * seats.min) + (seats.max * (input.length + 1))) / 2 - seats.sum
  }
}
