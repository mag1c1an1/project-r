pub fn pattern_decode(pattern: &str) -> Result<(u64, u64, u64), String> {
    let mut key: u64 = 0;
    let mut mask: u64 = 0;
    let mut shift: u64 = 0;
    let mut temp_shift: u64 = 0;

    for (i, c) in pattern.chars().enumerate() {
        match c {
            '0' => {
                key = key << 1;
                mask = (mask << 1) | 1;
                temp_shift = 0;
            }
            '1' => {
                key = (key << 1) | 1;
                mask = (mask << 1) | 1;
                temp_shift = 0;
            }
            '?' => {
                key = key << 1;
                mask = mask << 1;
                temp_shift += 1;
            }
            ' ' => continue, // 忽略空格
            _ => {
                return Err(format!(
                    "Invalid character '{}' at position {} in pattern string",
                    c, i
                ));
            }
        }

        shift = temp_shift; // 记录最后的连续 '?' 的数量
    }

    // 右移 `key` 和 `mask`，忽略末尾的 '?'
    Ok((key >> shift, mask >> shift, shift))
}
