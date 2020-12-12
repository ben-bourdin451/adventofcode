package days

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
import scala.util.matching.Regex

case class ConsoleState(acc: Int, index: Int) {}
object day8 {
  def extractValue(inst: String): Int = {
    val reg: Regex = """\w+ ([+\-])(\d+)""".r
    inst match {
      case reg(sign, n) => {
        if (sign == "-") {
          -Integer.valueOf(n)
        } else {
          Integer.valueOf(n)
        }
      }
    }
  }

  def runCode(in: Seq[String],
              s: ConsoleState,
              swap: Boolean): (Int, Boolean, mutable.Seq[ConsoleState]) = {
    val swappable: ArrayBuffer[ConsoleState] = ArrayBuffer[ConsoleState]()
    var acc = s.acc
    val m = mutable.Map[Int, Int]()
    var index = s.index
    while (index >= 0 && index < in.length && !m.contains(index)) {
      val inst = in(index)
      m.put(index, 1)
      inst match {
        case _ if inst.startsWith("acc") =>
          acc += extractValue(inst)
          index += 1
        case _ if inst.startsWith("jmp") =>
          swappable.prepend(ConsoleState(acc, index))
          if (swap && index == s.index) {
            index += 1
          } else {
            index += extractValue(inst)
          }
        case _ if inst.startsWith("nop") =>
          swappable.prepend(ConsoleState(acc, index))
          if (swap && index == s.index) {
            index += extractValue(inst)
          } else {
            index += 1
          }
      }
    }
    (acc, index < in.length - 1, swappable)
  }

  def part1(in: Seq[String]): Int = {
    runCode(in, ConsoleState(0, 0), swap = false)._1
  }

  def part2(in: Seq[String]): Int = {
    var swaps = runCode(in, ConsoleState(0, 0), swap = false)._3
    var res = runCode(in, swaps.head, swap = true)
    while (res._2) {
      res = runCode(in, swaps.head, swap = true)
      swaps = swaps.tail
    }
    res._1
  }
}
