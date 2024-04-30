use std::collections::HashMap;

use crate::common::{
    update_counts,
    calculate_counts,
    merge,
    bytes_to_u32,
    bytes_to_byte_string_literal,
};

const VOCAB_SIZE: usize = 512;
const NUM_MERGES: usize = VOCAB_SIZE - 256;

pub fn train(
    text: &str,
    merges: &mut HashMap<(u32, u32), u32>,
    vocab: &mut HashMap<u32, String>,
    counts: &mut HashMap<(u32, u32), u32>
) -> Vec<u32> {
    for i in 0..=255 {
        vocab.insert(i as u32, format!("{}", i as u8 as char));
    }
    let ids = text.as_bytes();
    let mut u32_ids = bytes_to_u32(&ids);

    // let mut idx: usize = 256;
    for i in 0..NUM_MERGES {
        *counts = calculate_counts(&u32_ids);
        let max_pair = counts
            .iter()
            .max_by_key(|entry| entry.1)
            .unwrap();
        let idx = 256 + i;
        u32_ids = merge(u32_ids, *max_pair.0, idx as u32);
        merges.insert(*max_pair.0, idx as u32);
        let merged_bytes =
            bytes_to_byte_string_literal(&vocab[&max_pair.0.0].as_bytes()) +
            &bytes_to_byte_string_literal(&vocab[&max_pair.0.1].as_bytes());
        vocab.insert(idx as u32, merged_bytes);
        println!(
            "Epoch {}/{}: {} {} -> {} ({:?}) had {:?} occurrences",
            i + 1,
            NUM_MERGES,
            max_pair.0.0,
            max_pair.0.1,
            idx,
            vocab.get(&(idx as u32)).unwrap(),
            counts.get(max_pair.0).unwrap()
        );
    }
    u32_ids
}

pub fn encode(text: &str, merges: &HashMap<(u32, u32), u32>) -> Vec<u32> {
    let text_bytes = text.as_bytes();
    let mut ids = bytes_to_u32(text_bytes);

    while ids.len() >= 2 {
        let mut stats = HashMap::new();
        update_counts(&ids, &mut stats);
        let min_pair = stats
            .iter()
            .min_by_key(|&(p, _)| merges.get(p).cloned().unwrap_or(std::u32::MAX))
            .unwrap().0;

        if !merges.contains_key(&min_pair) {
            break;
        }

        let idx = merges.get(&min_pair).unwrap();
        ids = merge(ids, *min_pair, *idx);
    }

    ids
}

pub fn decode(ids: &[u32], vocab: &HashMap<u32, String>) -> String {
    let mut text = String::new();
    for &id in ids {
        text.push_str(&vocab[&id]);
    }
    text
}
