use std::collections::{HashMap, HashSet};

struct PageRule {
    pub before: HashSet<i32>,
    pub after: HashSet<i32>,
}

pub fn part1(data: &Vec<String>) -> i32 {
    let mut iter = data.split(|s| s.is_empty());
    let rules: Vec<String> = iter.next().unwrap().to_vec();
    let page_orders: Vec<String> = iter.next().unwrap().to_vec();

    let page_rules = parse_rules(&rules);
    let mut count = 0;
    for page_order in page_orders {
        let page_order: Vec<i32> = page_order.split(",").map(|s| s.parse().unwrap()).collect();
        if is_page_order_valid(&page_rules, &page_order) {
            count += page_order[page_order.len() / 2];
        }
    }
    count
}

fn is_page_order_valid(page_rules: &HashMap<i32, PageRule>, page_order: &Vec<i32>) -> bool {
    for i in 0..page_order.len() {
        let rule = page_rules.get(&page_order[i]);
        if rule.is_none() {
            continue;
        }
        // check before
        for b in i..0 {
            if rule.unwrap().after.contains(&page_order[b]) {
                return false;
            }
        }
        // check after
        for a in i..page_order.len() {
            if rule.unwrap().before.contains(&page_order[a]) {
                return false;
            }
        }
    }
    true
}

fn parse_rules(rules: &Vec<String>) -> HashMap<i32, PageRule> {
    let mut page_rules: HashMap<i32, PageRule> = HashMap::new();
    for rule in rules {
        let mut parts = rule.split("|");

        let l: i32 = parts.next().unwrap().parse().unwrap();
        let r: i32 = parts.next().unwrap().parse().unwrap();

        let vl = page_rules.entry(l).or_insert(PageRule {
            before: HashSet::new(),
            after: HashSet::new(),
        });
        vl.after.insert(r);

        let vr = page_rules.entry(r).or_insert(PageRule {
            before: HashSet::new(),
            after: HashSet::new(),
        });
        vr.before.insert(l);
    }
    page_rules
}

pub fn part2(data: &Vec<String>) -> i32 {
    let mut iter = data.split(|s| s.is_empty());
    let rules: Vec<String> = iter.next().unwrap().to_vec();
    let page_orders: Vec<String> = iter.next().unwrap().to_vec();

    let page_rules = parse_rules(&rules);
    let mut count = 0;
    for page_order in page_orders {
        let page_order: Vec<i32> = page_order.split(",").map(|s| s.parse().unwrap()).collect();
        if !is_page_order_valid(&page_rules, &page_order) {
            let sorted_page_order = sort_page_order(&page_rules, &page_order);
            count += sorted_page_order[sorted_page_order.len() / 2];
        }
    }
    count
}

fn sort_page_order(page_rules: &HashMap<i32, PageRule>, page_order: &Vec<i32>) -> Vec<i32> {
    let mut sorted = page_order.clone();
    sorted.sort_by(|a, b| {
        let rule_a = page_rules.get(a);
        if rule_a.is_some() {
            let rule_a = rule_a.unwrap();
            if rule_a.before.contains(b) {
                return std::cmp::Ordering::Less;
            }
            if rule_a.after.contains(b) {
                return std::cmp::Ordering::Greater;
            }
        }

        let rule_b = page_rules.get(b);
        if rule_b.is_some() {
            let rule_b = rule_b.unwrap();
            if rule_b.before.contains(a) {
                return std::cmp::Ordering::Greater;
            }
            if rule_b.after.contains(a) {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal
    });
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("47|53"),
                String::from("97|13"),
                String::from("97|61"),
                String::from("97|47"),
                String::from("75|29"),
                String::from("61|13"),
                String::from("75|53"),
                String::from("29|13"),
                String::from("97|29"),
                String::from("53|29"),
                String::from("61|53"),
                String::from("97|53"),
                String::from("61|29"),
                String::from("47|13"),
                String::from("75|47"),
                String::from("97|75"),
                String::from("47|61"),
                String::from("75|61"),
                String::from("47|29"),
                String::from("75|13"),
                String::from("53|13"),
                String::from(""),
                String::from("75,47,61,53,29"),
                String::from("97,61,53,29,13"),
                String::from("75,29,13"),
                String::from("75,97,47,61,53"),
                String::from("61,13,29"),
                String::from("97,13,75,29,47"),
            ],
            want: 143,
        }];
        for c in cases {
            assert_eq!(part1(&c.given), c.want);
        }
    }

    #[test]
    fn day5_part2() {
        struct Case {
            given: Vec<String>,
            want: i32,
        }

        let cases: Vec<Case> = vec![Case {
            given: vec![
                String::from("47|53"),
                String::from("97|13"),
                String::from("97|61"),
                String::from("97|47"),
                String::from("75|29"),
                String::from("61|13"),
                String::from("75|53"),
                String::from("29|13"),
                String::from("97|29"),
                String::from("53|29"),
                String::from("61|53"),
                String::from("97|53"),
                String::from("61|29"),
                String::from("47|13"),
                String::from("75|47"),
                String::from("97|75"),
                String::from("47|61"),
                String::from("75|61"),
                String::from("47|29"),
                String::from("75|13"),
                String::from("53|13"),
                String::from(""),
                String::from("75,47,61,53,29"),
                String::from("97,61,53,29,13"),
                String::from("75,29,13"),
                String::from("75,97,47,61,53"),
                String::from("61,13,29"),
                String::from("97,13,75,29,47"),
            ],
            want: 123,
        }];
        for c in cases {
            assert_eq!(part2(&c.given), c.want);
        }
    }
}
