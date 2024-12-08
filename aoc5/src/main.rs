use std::{fs, io, collections::{HashMap, HashSet}};

type Rules = HashSet<i32>;
type RulesSet = HashMap<i32, Rules>;

type Update = Vec<i32>;

// check if slice does not contain any numbers in the forbidden nums
fn check_preceding_nums(
    update_slice: &[i32],
    idx_left: i8,
    forbidden_nums: &Rules
) -> bool {
    if idx_left < 0 { return true };
    if forbidden_nums.contains(&update_slice[idx_left as usize]) { false } 
    else { check_preceding_nums(update_slice, idx_left - 1, forbidden_nums) }
}

// check for num from the right if numbers left are not contained in the rules
fn check_rules(idx_right: usize, update: &Update, rules: &RulesSet) -> bool {
    if idx_right == 0 { return true };
    let empty_hashset: Rules = Rules::new();
    let forbidden_nums = rules.get(&update[idx_right]).unwrap_or(&empty_hashset); 
    if check_preceding_nums(&update[0..idx_right], idx_right as i8 - 1, forbidden_nums) {
        check_rules(idx_right - 1, update, rules)
    } else { false }
}

// not really needed but nice for readability i guess
fn update_is_valid(update: &Update, rules: &RulesSet) -> bool {
    let idx_right: usize = update.len() - 1;
    check_rules(idx_right, update, rules)
}

fn check_left(
    mut update: Update,
    idx_left: i8,
    idx_right: usize,
    rules_set: &RulesSet
) -> Update {
    if idx_left < 0 { return update };

    let empty_hashset: Rules = Rules::new();
    let rules = rules_set.get(&update[idx_right]).unwrap_or(&empty_hashset);
    if rules.contains(&update[idx_left as usize]) {
        let num = update.remove(idx_right);
        update.insert(idx_left as usize, num);
        update = check_left(update, idx_left - 1, idx_left as usize, rules_set)
    };
    check_left(update, idx_left - 1, idx_right, rules_set)
}

fn check_right(idx_right: usize, mut update: Update, rules_set: &RulesSet) -> Update {
    if idx_right == update.len() { return update };
    update = check_left(update, idx_right as i8 - 1, idx_right, rules_set);
    check_right(idx_right + 1, update, rules_set)
}

fn main() -> io::Result<()> {
    let inputs = fs::read_to_string("input")?;
    let mut input_iter = inputs.split("\n\n");

    let rules = input_iter.next().unwrap().to_owned();
    let rule_iter = rules.split_whitespace()
        .map(|x| x.split("|")
            .filter_map(|y| y.parse::<i32>().ok())
        );

    let updates = input_iter.next().unwrap().to_owned();
    let update_iter = updates.split_whitespace()
        .map(|x| x.split(",")
            .filter_map(|y| y.parse::<i32>().ok()).collect()
        );

    let rule_map = rule_iter
        .fold(RulesSet::new(), |mut map, x| {
            let key_value: Vec<i32> = x.collect();
            let key = key_value[0].to_owned();
            let value = key_value[1].to_owned();

            let set = map.get_mut(&key);
            match set {
                None => { 
                    let mut new_set = Rules::new();
                    new_set.insert(value);
                    map.insert(key, new_set);
                },
                Some(i) => { i.insert(value); }
            };
            map
        });

    let (good_updates, bad_updates): (Vec<Vec<i32>>, Vec<Vec<i32>>) = update_iter
        .partition(|x| {
            update_is_valid(x, &rule_map)
        });

    let total1: i32 = good_updates.into_iter().map(|x| {
        let mid_idx = x.len() / 2;
        x[mid_idx]
    }).sum();

    println!("{}", total1);

    let total2: i32 = bad_updates.into_iter().map(|x| {
        let update = check_right(1, x, &rule_map);
        let mid_idx = update.len() / 2;
        update[mid_idx]
    }).sum();

    println!("{}", total2);

    Ok(())
}
