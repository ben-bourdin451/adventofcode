package days

object day10 {
  def getDiffs(in: Seq[Int]): Seq[Int] = {
    var prev = 0
    in.map(e => {
      val diff = e - prev
      prev = e
      diff
    })
  }

  def fib3(n: Int): Int = {
    if (n <= 1) {
      1
    } else if (n == 2) {
      2
    } else {
      fib3(n - 1) + fib3(n - 2) + fib3(n - 3)
    }
  }

  def part1(in: Seq[Int]): Int = {
    val diffs: Seq[Int] = getDiffs(in.sortWith(_ < _))
    diffs.count(_ == 1) * (diffs.count(_ == 3) + 1)
  }

  def part2(in: Seq[Int]): BigInt = {
    val diffs: Seq[Int] = getDiffs(in.sortWith(_ < _))
    var i = 0
    var total = BigInt(1)
    var ones = 0
    while (i < diffs.length) {
      if (diffs(i) == 1) {
        ones += 1
      } else {
        total *= fib3(ones)
        ones = 0
      }
      i += 1
    }
    total * fib3(ones)
  }
}
