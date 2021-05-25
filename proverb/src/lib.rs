pub fn build_proverb(list: &[&str]) -> String {
    let last = list.len() - 1;

    list.iter()
        .enumerate()
        .map(|(i, v)| {
            let next = i + 1;
            if next <= last {
                return format!("For want of a {} the {} was lost.", v, list[next]);
            }
            return format!("And all for the want of a {}.", list.first().unwrap());
        })
        .collect::<Vec<String>>()
        .join("\n")
}
