package days

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

object day9 {
  def hasSum(in: Seq[BigInt], n: BigInt): Boolean = {
    in.combinations(2)
      .find(group => group.sum == n)
      .exists(p => { p.nonEmpty })
  }

  def part1(in: Seq[BigInt], preamble: BigInt): BigInt = {
    var data: ArrayBuffer[BigInt] = ArrayBuffer[BigInt]()

    var i = 0
    while (i < preamble) {
      data.append(in(i))
      i += 1
    }
    var break = false
    var res = BigInt(0)
    while (i < in.length && !break) {
      if (hasSum(data.toSeq, in(i))) {
        data = data.tail
        data.append(in(i))
      } else {
        res = in(i)
        break = true
      }
      i += 1
    }
    res
  }

  def findContiguous(in: Seq[BigInt], want: BigInt): (Int, Int) = {
    var i = 0
    var j = 1
    var sum = in(i) + in(j)
    while (i < j && j < in.length) {
      if (sum == want) {
        return (i, j)
      } else if (sum > want) {
        sum -= in(i)
        i += 1
      } else {
        j += 1
        sum += in(j)
      }
    }
    (0, 0)
  }

  def part2(in: Seq[BigInt], want: BigInt): BigInt = {
    val indices = findContiguous(in, want)
    val arr = in.slice(indices._1, indices._2 + 1)
    arr.min + arr.max
  }
}
