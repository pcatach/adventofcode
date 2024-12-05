
/* Part 1: given page ordering rules such as
```
1|2
2|4
2|5
```
X|Y means that X has to come before Y

also given "page updates"
```
1,2,3,4,5
```
need to know which updates are in the right order
and sum the middle page numbers of those

build a graph of rules, i.e. a map X -> [Y, Z, W]
for each rule X|Y, X|Z, X|Z
then for each page X iterate in the remaining pages.
If one of the following pages has X in their rule, bail
otherwise, it's in the right order
 */
use std::collections::HashMap;
use std::cmp::Ordering;
use std::io;

use utils::read_from_args;

fn main() -> io::Result<()> {
    let input = read_from_args()?;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut lines = input.lines();

    // build rules
    let mut line = lines.next();
    while line != Some("") {
        let rule = line.unwrap().split_once("|").unwrap();
        let (x, y): (u32, u32) = (rule.0.parse().unwrap(), rule.1.parse().unwrap());
        match rules.get_mut(&x) {
            Some(pages) => {
                pages.push(y); 
            },
            None => {
                let mut pages = Vec::new();
                pages.push(y);
                rules.insert(x, pages);
            }
        }
        line = lines.next();
    }
    dbg!(&rules);
    line = lines.next();

    // parse updates
    let mut sum_for_correct_updates = 0;
    let mut sum_for_incorrect_updates = 0;
    while line.is_some() {
        let mut pages: Vec<u32> = line.unwrap().split(",").map(|page| page.parse().unwrap()).collect();

        let update_is_correct = pages.windows(2).all(
            |w| {
                let (page1, page2) = (w[0], w[1]);
                rules.get(&page1).unwrap_or(&Vec::new()).contains(&page2)
            }
        );

        pages.sort_by(|a, b| {
            match rules.get(a) {
                Some(allowed_successors) => match allowed_successors.contains(b) {
                    true => Ordering::Less,
                    false => Ordering::Greater
                },
                None => Ordering::Greater
            }
        });

        let middle_page = pages[pages.len() / 2];
        dbg!(pages);
        if update_is_correct {
            sum_for_correct_updates += middle_page;
        } else {
            sum_for_incorrect_updates += middle_page;
        }
        line = lines.next()
    }
    dbg!(sum_for_correct_updates, sum_for_incorrect_updates);
    Ok(())
}
