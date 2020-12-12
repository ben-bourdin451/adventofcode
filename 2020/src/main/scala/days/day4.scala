package days

import java.util

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
import scala.io.Source
import scala.util.matching.Regex

object day4 {
  def loadPassportsFromFile(filename: String): mutable.Seq[String] = {
    val bufferedSource = Source.fromFile(filename)
    val lines = bufferedSource.getLines
    val passports: ArrayBuffer[String] = ArrayBuffer[String]()
    var word = ""
    lines.foreach(line => {
      if (line != "") {
        word += line + " "
      } else {
        passports += word.trim()
        word = ""
      }
    })
    passports += word.trim()
    bufferedSource.close
    passports
  }

  def parsePassport(p: String): Map[String, String] = {
    p.split(" ")
      .map(pairs => {
        val sp = pairs.split(":")
        Map(sp(0) -> sp(1))
      })
      .reduce(_ ++ _)
  }

  def checkMinMax(s: Option[String], min: Int, max: Int): Boolean = {
    val reg: Regex = """(\d{4})""".r
    s.exists {
      case reg(year) =>
        min <= Integer.valueOf(year) && Integer.valueOf(year) <= max
      case _ => false
    }
  }

  def checkReg(s: Option[String], r: Regex): Boolean = {
    s.exists {
      case r(_) => true
      case _    => false
    }
  }

  // hgt (Height) - a number followed by either cm or in:
  //  If cm, the number must be at least 150 and at most 193.
  //  If in, the number must be at least 59 and at most 76.
  def checkHeight(s: Option[String]): Boolean = {
    val reg: Regex = """(\d+)(cm|in)""".r
    s.exists {
      case reg(n, m) if m == "cm" =>
        150 <= Integer.valueOf(n) && Integer.valueOf(n) <= 193
      case reg(n, m) if m == "in" =>
        59 <= Integer.valueOf(n) && Integer.valueOf(n) <= 76
      case _ => false
    }
  }

  // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
  def checkHair(s: Option[String]): Boolean = {
    checkReg(s, """#([0-9a-f]{6})""".r)
  }

  // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
  def checkEye(s: Option[String]): Boolean = {
    s.exists(
      Map(
        "amb" -> (),
        "blu" -> (),
        "brn" -> (),
        "gry" -> (),
        "grn" -> (),
        "hzl" -> (),
        "oth" -> ()
      ).contains(_)
    )
  }

  // pid (Passport ID) - a nine-digit number, including leading zeroes.
  def checkPid(s: Option[String]): Boolean = {
    checkReg(s, """([0-9]{9})""".r)
  }

  def part1(input: mutable.Seq[String]): Int = {
    input
      .map(parsePassport)
      .count(p => {
        p.contains("byr") &&
        p.contains("iyr") &&
        p.contains("eyr") &&
        p.contains("hgt") &&
        p.contains("hcl") &&
        p.contains("ecl") &&
        p.contains("pid")
      })
  }

  def part2(input: mutable.Seq[String]): Int = {
    input
      .map(parsePassport)
      .count(p => {
        //  byr (Birth Year) - four digits; at least 1920 and at most 2002.
        checkMinMax(p.get("byr"), 1920, 2002) &&
        //  iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        checkMinMax(p.get("iyr"), 2010, 2020) &&
        //  eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        checkMinMax(p.get("eyr"), 2020, 2030) &&
        checkHeight(p.get("hgt")) &&
        checkHair(p.get("hcl")) &&
        checkEye(p.get("ecl")) &&
        checkPid(p.get("pid"))
      })
  }
}
