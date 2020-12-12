package days

import scala.collection.mutable
import scala.util.matching.Regex

case class Edge(n: Int, next: Node) {
  def getNext: Node = next
}
case class Node(name: String, var edges: Seq[Edge])

object day7 {
  val reg: Regex = """(\d+) ([\w\s]+)""".r

  def parse(input: Seq[String]): mutable.Map[String, Node] = {
    val m = mutable.Map[String, Node]()
    input.foreach(l => {
      if (!l.contains("no other bag")) {
        val bags: Seq[String] = l
          .replaceAll("""bag(s)?(\.)?""", "")
          .replaceAll(" contain", ",")
          .split(",")

        val name = bags.head.trim()
        val n: Node = m.getOrElse(name, {
          val n = Node(name, List[Edge]())
          m.put(name, n)
          n
        })

        n.edges = bags.tail.map(_.trim).map {
          case reg(num, bag) =>
            Edge(Integer.valueOf(num), m.getOrElse(bag, {
              val n = Node(bag, List[Edge]())
              m.put(bag, n)
              n
            }))
        }
      }
    })
    m
  }

  def countShinyGoldBags(n: Node): Int = {
    var count = 0
    if (n.name == "shiny gold") {
      1
    } else {
      n.edges.foreach(e => {
        count += countShinyGoldBags(e.next)
      })
      count
    }
  }

  def countBags(n: Node): Int = {
    var count = 0
    n.edges.foreach(e => {
      count += e.n + e.n * countBags(e.next)
    })
    count
  }

  def part1(in: Seq[String]): Int = {
    parse(in)
      .map(t => {
        if (t._1 != "shiny gold" && countShinyGoldBags(t._2) > 0) {
          1
        } else {
          0
        }
      })
      .sum
  }

  def part2(in: Seq[String]): Int = {
    countBags(parse(in)("shiny gold"))
  }
}
