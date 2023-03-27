use std::collections::HashMap;
use rayon::prelude::*;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let mut freqs = HashMap::new();

    let chunk_size = (input.len() + worker_count - 1) / worker_count; // round up
    let results: Vec<HashMap<char, usize>> = input
        .iter()
        .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>()
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut freqs = HashMap::new();
            for c in chunk {
                *freqs.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
            }
            freqs
        })
        .collect();

    for freq in results {
        for (c, count) in freq {
            *freqs.entry(c).or_insert(0) += count;
        }
    }

    freqs
}
