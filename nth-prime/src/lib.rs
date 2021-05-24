pub fn nth(n: u32) -> u32 {
    let nth_len = (n + 1) as usize;
    let mut nths = Vec::with_capacity(nth_len);

    let mut number = 1;
    loop {
        if nths.len() == nth_len {
            break;
        }

        number += 1;
        if nths.is_empty() {
            nths.push(number);
            continue;
        }

        if let Some(_r) = nths
            .iter()
            .find_map(|n| (number % n == 0).then(|| Some(())))
        {
            continue;
        }

        nths.push(number);
    }

    *nths.get(n as usize).unwrap()
}
