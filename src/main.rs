use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const VOCAB_SIZE: usize = 512;
const NUM_MERGES: usize = VOCAB_SIZE - 256;

fn get_counts(ids: &Vec<u32>, counts: &mut HashMap<(u32, u32), u32>) {
    let mut i: usize = 0;
    while i < ids.len() - 1 {
        counts
            .entry((ids[i], ids[i + 1]))
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(0);
        i += 1;
    }
}

fn get_counts_2(ids: &Vec<u32>) -> HashMap<(u32, u32), u32> {
    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();
    let mut i: usize = 0;
    while i < ids.len() - 1 {
        counts
            .entry((ids[i], ids[i + 1]))
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(0);
        i += 1;
    }
    counts
}

fn merge(ids: Vec<u32>, pair: (u32, u32), idx: u32) -> Vec<u32> {
    let mut new_ids_vec: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < ids.len() {
        if i < ids.len() - 1 && ids[i] == pair.0 && ids[i + 1] == pair.1 {
            new_ids_vec.push(idx);
            i += 2;
        } else {
            new_ids_vec.push(ids[i]);
            i += 1;
        }
    }

    new_ids_vec
}

fn bytes_to_u32(ids: &[u8]) -> Vec<u32> {
    let mut u32_ids = Vec::new();

    for byte in ids.iter() {
        let u32_id: u32 = *byte as u32;
        u32_ids.push(u32_id);
    }

    u32_ids
}

fn bytes_to_byte_string_literal(bytes: &[u8]) -> String {
    format!(
        "{}",
        bytes
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
    )
}

fn get_dataset() -> String {
    let data = fs::read_to_string("./myresponse.json").expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
    let mut text = String::new();

    if let Some(rows) = json.get("rows").and_then(serde_json::Value::as_array) {
        for row in rows {
            if let Some(row_obj) = row.get("row").and_then(serde_json::Value::as_object) {
                if let Some(text_val) = row_obj.get("text").and_then(serde_json::Value::as_str) {
                    text.push_str(text_val);
                }
            }
        }
    }
    text
}

fn train(
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
        *counts = get_counts_2(&u32_ids);
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

fn encode(text: &str, merges: &HashMap<(u32, u32), u32>) -> Vec<u32> {
    let text_bytes = text.as_bytes();
    let mut ids = bytes_to_u32(text_bytes);

    while ids.len() >= 2 {
        let mut stats = HashMap::new();
        get_counts(&ids, &mut stats);
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

fn decode(ids: &[u32], vocab: &HashMap<u32, String>) -> String {
    let mut text = String::new();
    for &id in ids {
        text.push_str(&vocab[&id]);
    }
    text
}

fn main() {
    let now = Instant::now();
    let text = get_dataset();
    let text_ref: &str = &text;

    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();
    let mut merges: HashMap<(u32, u32), u32> = HashMap::new();
    let mut vocab: HashMap<u32, String> = HashMap::new();

    let ids = text.as_bytes();

    let u32_ids = train(text_ref, &mut merges, &mut vocab, &mut counts);

    let elapsed = now.elapsed();
    println!("Training Elapsed: {:.2?}", elapsed);

    let encoding = encode("I am loved by many of my followers", &merges);
    print!("\nEncoded Value: ");
    for item in encoding.iter() {
        print!("{},", item);
    }

    let decoding = decode(&encoding, &vocab);
    println!("\nDecoding: {}", decoding);

    println!(
        "Compression {}/{} = {:.3}x",
        ids.len(),
        u32_ids.len(),
        (ids.len() as f32) / (u32_ids.len() as f32)
    );
}
