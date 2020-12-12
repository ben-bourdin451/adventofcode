package days

import scala.io.Source
import scala.util.matching.Regex

object fileUtils {
  def loadFile(name: String): Seq[String] = {
    val bufferedSource = Source.fromFile(name)
    val lines = bufferedSource.getLines.toSeq
    bufferedSource.close
    lines
  }
}

case class Password(min: Int, max: Int, char: Char, pwd: String) {}
object day2 {
  val reg: Regex = """(\d+)-(\d+) (\w): (\w+)""".r
  def parse(in: String): Password = {
    in match {
      case reg(min, max, c, pwd) =>
        Password(Integer.valueOf(min), Integer.valueOf(max), c.head, pwd)
    }
  }

  def solve(input: Seq[String], pred: Password => Boolean): Int = {
    input
      .map(parse)
      .collect({
        case p: Password if pred(p) => p
      })
      .length
  }

  def part1(p: Password): Boolean = {
    val c = p.pwd.count(c => c == p.char)
    p.min <= c && c <= p.max
  }

  def part2(p: Password): Boolean = {
    (p.pwd(p.min - 1) == p.char && p.pwd(p.max - 1) != p.char) ||
    (p.pwd(p.min - 1) != p.char && p.pwd(p.max - 1) == p.char)
  }
}
