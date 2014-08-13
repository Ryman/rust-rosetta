// http://rosettacode.org/wiki/Run-length_encoding
static INPUT : &'static str = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";

// Needed so look-and-say_sequence compiles cleanly, because it
// uses this code as a library
#[allow(dead_code)]
#[cfg(not(test))]
fn main() {
    let enc = encode(INPUT);
    println!("encoded {}", enc);

    let dec = decode(enc.as_slice());
    println!("decoded {}", dec.unwrap());
}

pub fn encode(value: &str) -> String {
    let mut ret = String::new();
    let mut chars = value.chars();

    let (mut count, mut cur) = (1u, chars.next());
    if cur.is_none() { return ret }

    for chr in chars {
        if cur == Some(chr) {
            count += 1
        } else {
            ret.push_str(count.to_string().as_slice());
            ret.push_char(cur.unwrap());
            count = 1u;
            cur = Some(chr);
        }
    }
    ret.push_str(count.to_string().as_slice());
    ret.push_char(cur.unwrap());
    ret
}

pub fn decode(value: &str) -> Result<String, String> {
    let mut start = 0;
    let mut result = String::new();
    if value.is_empty() { return Ok(result) }

    for (i, c) in value.char_indices() {
        if c.is_digit() { continue }
        if i == start { return Err(format!("expected digit, found {}", c)) }

        let count = value.slice(start, i);
        match from_str::<uint>(count) {
            Some(count) => {
                for _ in range(0, count) {
                    result.push_char(c);
                }

                start = i + 1;
            }
            None => return Err(format!("Failed to parse integer: {}", count))
        }
    }

    Ok(result)
}

#[test]
fn test_failed_decode() {
    let s = "34028236692093846346337460743176821145567654W";
    // This number is too large for uint parsing.
    assert!(decode(s).is_err());
}

#[test]
fn test_encode_decode() {
    assert_eq!(decode(encode(INPUT).as_slice()).unwrap(), INPUT.to_string());
    assert_eq!(decode("a"), Err("expected digit, found a".to_string()));
}
