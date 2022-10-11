use std::{io::{BufRead, self}, error};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    roll: usize,
    prev: Option<Box<Node>>,
}

impl Node {
    fn new(id: usize, roll: usize, prev: Option<Box<Node>>) -> Self {
        Self { id, roll, prev }
    }
}

fn read_num_pair<R>(mut reader: R) -> (usize, usize)
where
    R: BufRead
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();

    match buffer.trim().split_whitespace().collect::<Vec<&str>>().as_slice() {
        &[n, m] => (n.parse().unwrap(), m.parse().unwrap()),
        _ => panic!("never"),
    }
}

fn get_path(num_tiles: usize, game_board: &mut Vec<(usize, bool)>) -> Option<Node> {
    let mut gen: Vec<_> = vec![Node::new(0, 0, None)];
    while gen.len() > 0 {
        let mut temp = Vec::new();
        for node in gen {
            for n in 1..7 {
                let new_id = node.id + n;
                if new_id >= num_tiles {
                    continue;
                } else {
                    let (q, visited) = game_board.get_mut(new_id).unwrap();
                    if *visited {
                        continue;
                    } else if *q == (num_tiles - 1) {
                        return Some(Node::new(*q, n, Some(Box::new(node.clone()))));
                    } else {
                        *visited = true;
                        temp.push(Node::new(*q, n, Some(Box::new(node.clone()))));
                    }
                }
            }
        }
        gen = temp;
    }
    None
}

fn tsirkus<R>(mut reader: R) -> Result<Vec<usize>>
where
    R: BufRead,
{
    let (num_tiles, m) = read_num_pair(&mut reader);

    let mut game_board: Vec<_> = (0..num_tiles).map(|i| (i, false)).collect();

    for _ in 0..m {
        let (a, l) = read_num_pair(&mut reader);
        if let Some(m) = game_board.get_mut(a - 1) {
            (*m) = (l - 1, false);
        }
    }

    let mut rolls = Vec::new();

    match get_path(num_tiles, &mut game_board) {
        Some(path) => {
            let mut cur = Box::new(path);
            while let Some(prev) = cur.prev {
                rolls.push(cur.roll);
                cur = prev;
            }
        },
        None => {}
    }

    rolls.reverse();

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