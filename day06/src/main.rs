use std::io::stdin;

fn main() {
    for line in stdin().lines().flatten() {
        println!(
            "sop: {:?} , som: {:?}",
            find_first_after_marker(&line, 4),
            find_first_after_marker(&line, 14)
        );
    }
}

fn find_first_after_marker(input: &str, window_size: usize) -> Option<usize> {
    let x = input.chars().collect::<Vec<char>>();
    x.windows(window_size)
        .enumerate()
        .find_map(|(idx, slice)| {
            if (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1])) {
                None
            } else {
                Some(idx + window_size)
            }
        })
        .into_iter()
        .next()
}
