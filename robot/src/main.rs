use std::{io::{BufRead, self}, error, collections::HashMap};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn get_diff(map: &HashMap<char, i16>, ch1: char, ch2: char) -> i16 {
    match (map.get(&ch1), map.get(&ch2)) {
        (Some(a), Some(b)) => (a - b).abs(),
        _ => 0
    }
}

fn robot<R>(mut reader: R) -> Result<i16>
where
    R: BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut direction_map = HashMap::new();
    for ch in ['N', 'S', 'W', 'E'] {
        direction_map.insert(ch, 0i16);
    }

    for ch in buffer.chars() {
        direction_map.entry(ch)
            .and_modify(|count| *count += 1);
    }

    let diff_x = get_diff(&direction_map, 'E', 'W');
    let diff_y = get_diff(&direction_map, 'N', 'S');

    Ok(diff_x + diff_y)
}

fn main() -> Result<()> {
    let stdio = io::stdin();
    let input = stdio.lock();

    let result = robot(input)?;
    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::robot;

    #[test]
    fn first_sample() {
        let input = br#"5
NNEEE
"#;

        let result = robot(&input[..]).unwrap();

        assert_eq!(result, 5)
    }

    #[test]
    fn second_sample() {
        let input = br#"7
NNNSSWE
"#;

        let result = robot(&input[..]).unwrap();

        assert_eq!(result, 1)
    }
}