package days

import scala.collection.immutable.Range
import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
import scala.io.Source

object day6 {
  def loadInput(filename: String): ArrayBuffer[ArrayBuffer[String]] = {
    val bufferedSource = Source.fromFile(filename)
    val lines = bufferedSource.getLines
    val groups: ArrayBuffer[ArrayBuffer[String]] =
      ArrayBuffer[ArrayBuffer[String]]()
    var word = ArrayBuffer[String]()
    lines.foreach(line => {
      if (line != "") {
        word += line
      } else {
        groups += word
        word = ArrayBuffer()
      }
    })
    groups += word
    bufferedSource.close
    groups
  }

  def part1(in: ArrayBuffer[ArrayBuffer[String]]): Int = {
    in.map(_.mkString("").toSet.size).sum
  }

  def part2(in: ArrayBuffer[ArrayBuffer[String]]): Int = {
    in.map(_.reduce((x, y) => {
        x.toCharArray.filter(y.toSeq.contains(_)).mkString("")
      }).length)
      .sum
  }
}
