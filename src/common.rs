use std::collections::HashMap;

pub fn update_counts(ids: &Vec<u32>, counts: &mut HashMap<(u32, u32), u32>) {
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

pub fn calculate_counts(ids: &Vec<u32>) -> HashMap<(u32, u32), u32> {
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

pub fn merge(ids: Vec<u32>, pair: (u32, u32), idx: u32) -> Vec<u32> {
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

pub fn bytes_to_u32(ids: &[u8]) -> Vec<u32> {
    let mut u32_ids = Vec::new();

    for byte in ids.iter() {
        let u32_id: u32 = *byte as u32;
        u32_ids.push(u32_id);
    }

    u32_ids
}

pub fn bytes_to_byte_string_literal(bytes: &[u8]) -> String {
    format!(
        "{}",
        bytes
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
    )
}
