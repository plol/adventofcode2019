
pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        4
    }
    fn main1(_input: &Vec<String>) -> String {
        let mut answer = 0;
        for i in 347312..805915 {
            let x = format!("{:06}", i);
            let mut matches = true;

            let mut has_duplicate = false;
            let mut prev = '\0';
            for d in x.chars() {
                if d == prev {
                    has_duplicate = true;
                }
                if d < prev {
                    matches = false;
                }
                prev = d;
            }
            matches = matches && has_duplicate;

            if matches {
                answer += 1;
            }
        }
        format!("{}", answer)
    }
    fn main2(_input: &Vec<String>) -> String {
        let mut answer = 0;
        for i in 347312..805915 {
            let x = format!("{:06}", i);
            let mut matches = true;

            let mut duplicates = std::collections::HashSet::new();
            let mut meh = std::collections::HashSet::new();
            let mut prev = '\0';
            for d in x.chars() {
                if d == prev {
                    if duplicates.contains(&d) {
                        meh.insert(d);
                    } else {
                        duplicates.insert(d);
                    }
                }
                if d < prev {
                    matches = false;
                }
                prev = d;
            }
            matches = matches && duplicates.difference(&meh).count() > 0;

            if matches {
                answer += 1;
            }
        }
        format!("{}", answer)
    }
}
