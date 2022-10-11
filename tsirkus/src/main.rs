use std::{io::{BufRead, self}, error};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn tsirkus<R>(mut reader: R) -> Result<Vec<u16>>
where
    R: BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let rolls = Vec::new();

    Ok(rolls)
}

fn main() -> Result<()> {
    let stdio = io::stdin();
    let input = stdio.lock();

    let result = tsirkus(input)?;
    if result.len() == 0 {
        println!("EI SAA");
    } else {
        println!("{}", result.len());
        let string_list: Vec<_> = result.iter().map(|x| x.to_string()).collect();
        println!("{}", string_list.join(" "));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tsirkus;

    #[test]
    fn first_sample() {
        let input = br#"30 0
"#;

        let result = tsirkus(&input[..]).unwrap();

        assert_eq!(result.len(), 5);
        assert_eq!(result, vec![5, 6, 6, 6, 6]);
    }

    #[test]
    fn second_sample() {
        let input = br#"7 1
2 7
"#;

        let result = tsirkus(&input[..]).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn third_sample() {
        let input = br#"31 6
5 2
6 2
7 2
8 2
9 2
10 2
"#;

        let result = tsirkus(&input[..]).unwrap();

        assert_eq!(result.len(), 0);
    }
}