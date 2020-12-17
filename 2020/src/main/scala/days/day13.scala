package days

object day13 {

  def part1(timestamp: Int, buses: Seq[Int]): Int = {
    val times = buses.map(b => {
      val m = timestamp % b
      if (m > 0) {
        (b - m, b)
      } else {
        (m, b)
      }
    })

    val next = times.minBy(_._1)
    next._1 * next._2
  }

  def egcd(a: BigInt, b: BigInt): BigInt = {
    var r1: BigInt = a
    var r2: BigInt = b
    var u1: BigInt = 1
    var u2: BigInt = 0
    var v1: BigInt = 0
    var v2: BigInt = 1
    while (r2 != 0) {
      val q: BigInt = r1 / r2
      val r3 = r1
      val u3 = u1
      val v3 = v1

      r1 = r2
      u1 = u2
      v1 = v2

      r2 = r3 - (q * r2)
      u2 = u3 - (q * u2)
      v2 = v3 - (q * v2)
    }

    if (a * u1 + b * v1 != 1) {
      throw new RuntimeException("GCD is not 1")
    }
    u1
  }

  def part2(buses: Seq[String]): BigInt = {
    var p = BigInt(1)
    buses.foreach(b => {
      if (b != "x") {
        p *= BigInt(b)
      }
    })

    var i = 0
    var t = BigInt(0)
    while (i < buses.length) {
      if (buses(i) != "x") {
        val mod = BigInt(buses(i))
        val n = p / mod
        t += -i * egcd(n, mod) * n
      }
      i += 1
    }

    while (t < 0) {
      t += p
    }

    t % p
  }
}
