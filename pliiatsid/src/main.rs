use std::{io::{BufRead, self}, error};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn read_ranges(buffer: &String) -> Vec<(char, u16, u16)> {
    let mut ranges = Vec::new();
    let mut current_range = ('-', 1, 1);
    for (i, ch) in buffer.char_indices() {
        if ch != 't' && ch != 'n' {
            ranges.push((current_range.0, current_range.1 as u16, current_range.2 as u16));
            break
        } else if i == 0 {
            current_range = (ch, 1, 1);
            continue;
        } else if current_range.0 == ch {
            current_range = (current_range.0, current_range.1, i + 1);
            continue;
        } else {
            ranges.push((current_range.0, current_range.1 as u16, current_range.2 as u16));
            current_range = (ch, i + 1, i + 1);
        }
    }
    ranges
}

fn pliiatsid<R>(mut reader: R) -> Result<Vec<(u16, u16)>>
where
    R: BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let ranges = read_ranges(&buffer);

    let ch = ranges[0].0;
    let flips: Vec<_> = ranges.iter().filter(|x| x.0 != ch).map(|x| (x.1, x.2)).collect();

    Ok(flips)
}

fn main() -> Result<()> {
    let stdio = io::stdin();
    let input = stdio.lock();

    let result = pliiatsid(input)?;
    println!("{}", result.len());
    for (a, b) in result {
        println!("{}-{}", a, b);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::pliiatsid;

    #[test]
    fn sample() {
        let input = br#"10
tttnntnttt
"#;

        let result = pliiatsid(&input[..]).unwrap();

        assert_eq!(result.len(), 2);

        let (a, b) = result.get(0).unwrap();
        assert_eq!(*a, 4);
        assert_eq!(*b, 5);

        let (a, b) = result.get(1).unwrap();
        assert_eq!(*a, 7);
        assert_eq!(*b, 7);
    }

    #[test]
    fn no_flips() {
        let input = br#"10
tttttttttt
"#;

        let result = pliiatsid(&input[..]).unwrap();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn asymmetric() {
        let input = br#"10
ttttttnnnn
"#;

        let result = pliiatsid(&input[..]).unwrap();

        assert_eq!(result.len(), 1);

        let (a, b) = result.get(0).unwrap();
        assert_eq!(*a, 7);
        assert_eq!(*b, 10);
    }

    #[test]
    fn sample_1() {
        let input = br#"10
tntntntntn
"#;

        let result = pliiatsid(&input[..]).unwrap();

        result.iter().for_each(|x| println!("R: {:?}", x));

        assert_eq!(result.len(), 5);

        let (a, b) = result.get(0).unwrap();
        assert_eq!(*a, 2);
        assert_eq!(*b, 2);

        let (a, b) = result.get(1).unwrap();
        assert_eq!(*a, 4);
        assert_eq!(*b, 4);

        let (a, b) = result.get(2).unwrap();
        assert_eq!(*a, 6);
        assert_eq!(*b, 6);

        let (a, b) = result.get(3).unwrap();
        assert_eq!(*a, 8);
        assert_eq!(*b, 8);

        let (a, b) = result.get(4).unwrap();
        assert_eq!(*a, 10);
        assert_eq!(*b, 10);
    }
}