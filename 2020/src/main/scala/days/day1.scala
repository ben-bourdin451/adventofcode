package days

class day1 {
  def part1(input: Seq[Int]): Int = {
    input
      .flatMap(a => {
        input.filter(b => a + b == 2020)
      })
      .product
  }

  def part2(input: List[Int]): Int = {
    val m = input.map(t => t -> t).toMap
    input.indices
      .flatMap(
        a =>
          input.indices
            .filter(b => {
              val want = 2020 - (input(a) + input(b))
              b > a && m.contains(want)
            })
            .map(b => {
              val want = 2020 - (input(a) + input(b))
              input(a) * input(b) * want
            })
      )
      .last
  }
}
