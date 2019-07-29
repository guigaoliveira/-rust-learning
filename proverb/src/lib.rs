use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        _ => list
            .windows(2)
            .map(|e| format!("For want of a {} the {} was lost.", e[0], e[1]))
            .chain(once(format!("And all for the want of a {}.", list[0])))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}
