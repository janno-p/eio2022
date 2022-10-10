use std::{io::{BufRead, self}, error};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn pliiatsid<R>(mut reader: R) -> Result<Vec<(u16, u16)>>
where
    R: BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    Ok(Vec::new())
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
        assert_eq!(*b, 7);

        let (a, b) = result.get(1).unwrap();
        assert_eq!(*a, 6);
        assert_eq!(*b, 6);
    }
}