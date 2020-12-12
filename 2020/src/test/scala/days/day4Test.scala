package days

import org.specs2.mutable.Specification

import scala.collection.mutable

class day4Test extends Specification {
  val input: mutable.Seq[String] =
    day4.loadPassportsFromFile("src/test/scala/days/day4_input.txt")

  val ex1: mutable.Seq[String] = Array(
    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
    "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929",
    "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm",
    "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"
  )

  "part 1" should {
    day4.part1(ex1) === 2
    day4.part1(input) === 254
  }

  "check min max" should {
    day4.checkMinMax(None, 1920, 2002) === false
    day4.checkMinMax(Some("1900"), 1920, 2002) === false
    day4.checkMinMax(Some("2000"), 1920, 2002) === true
    day4.checkMinMax(Some("2020"), 1920, 2002) === false
    day4.checkMinMax(Some("1967"), 2020, 2030) === false
  }

  "check height" should {
    day4.checkHeight(None) === false
    day4.checkHeight(Some("149cm")) === false
    day4.checkHeight(Some("194cm")) === false
    day4.checkHeight(Some("160cm")) === true
    day4.checkHeight(Some("55in")) === false
    day4.checkHeight(Some("77in")) === false
    day4.checkHeight(Some("65in")) === true
  }

  "check hair color" should {
    day4.checkHair(None) === false
    day4.checkHair(Some("#23ab99aba89a8ba98ba8")) === false
    day4.checkHair(Some("#42b3b")) === false
    day4.checkHair(Some("#425b3g")) === false
    day4.checkHair(Some("#425f3b")) === true
  }

  "check eye color" should {
    day4.checkEye(None) === false
    day4.checkEye(Some("amb")) === true
    day4.checkEye(Some("blu")) === true
    day4.checkEye(Some("brn")) === true
    day4.checkEye(Some("gry")) === true
    day4.checkEye(Some("grn")) === true
    day4.checkEye(Some("hzl")) === true
    day4.checkEye(Some("oth")) === true
    day4.checkEye(Some("other")) === false
    day4.checkEye(Some("asd")) === false
  }

  "check pid" should {
    day4.checkPid(None) === false
    day4.checkPid(Some("asfds")) === false
    day4.checkPid(Some("9")) === false
    day4.checkPid(Some("012340293402941093")) === false
    day4.checkPid(Some("123456789")) === true
    day4.checkPid(Some("000000009")) === true
  }

  "part 2" should {

    day4.part2(
      Array(
        "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
        "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023pid:3556412378 byr:2007"
      )
    ) === 0

    day4.part2(
      Array(
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
        "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
      )
    ) === 4

    day4.part2(input) === 184
  }
}
