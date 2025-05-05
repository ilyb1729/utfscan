fn utf8(arr: Vec<u8>) -> Result<Vec<u32>, Vec<u32>> {
    let mut it = arr.iter();
    let mut warn = false;
    let mut ans: Vec<u32> = Vec::new();

    'outer: while let Some(byte) = it.next() {
        if (byte >> 7) & 1 == 0 {
            ans.push(*byte as u32);
            continue;
        }

        let mut length = 0;
        'inner: {
            for i in (4..=6).rev() {
                if (byte >> i) & 1 == 0 {
                    break 'inner;
                }
                length += 1;
            }
            if (byte >> 3) & 1 == 1 {
                warn = true;
                continue 'outer;
            }
        }

        let shift = 2 + length;
        let mut cur = ((byte << shift) >> shift) as u32;

        if cur == 0 || length == 0 {
            warn = true;
            continue;
        }

        for _ in 0..length {
            if let Some(byte) = it.next() {
                cur <<= 6;
                cur += ((byte << 2) >> 2) as u32;
            } else {
                warn = true;
                continue 'outer;
            }
        }

        ans.push(cur);
    }

    if !warn { Ok(ans) } else { Err(ans) }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii() {
        let v1 = vec![1, 2, 3];
        assert_eq!(utf8(v1), Ok(vec![1, 2, 3]));
    }
}
