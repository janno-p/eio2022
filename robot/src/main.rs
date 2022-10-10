use std::{io::{BufRead, self}, error};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn robot<R>(mut reader: R) -> Result<u16>
where
    R: BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let num_steps: u16 = buffer.trim().parse()?;

    Ok(num_steps)
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