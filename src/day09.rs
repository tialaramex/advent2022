use jungle::readfile;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Default)]
struct Position {
    row: isize,
    col: isize,
}

fn touching(head: &Position, tail: &Position) -> bool {
    (head.row <= tail.row + 1)
        && (head.row >= tail.row - 1)
        && (head.col <= tail.col + 1)
        && (head.col >= tail.col - 1)
}

fn catch(head: &Position, tail: &mut Position) {
    if touching(head, tail) {
        return;
    }
    if head.row > tail.row {
        tail.row += 1;
    }
    if head.row < tail.row {
        tail.row -= 1;
    }
    if head.col > tail.col {
        tail.col += 1;
    }
    if head.col < tail.col {
        tail.col -= 1;
    }
}

use std::collections::HashSet;

pub fn a() {
    let ctxt = readfile("09");

    let mut head: Position = Default::default();
    let mut tail: Position = Default::default();

    let mut trail = HashSet::<Position>::new();
    trail.insert(tail);

    for line in ctxt.lines() {
        let (d, r) = line.split_once(' ').unwrap();
        let r: u32 = r.parse().expect("The right hand side should be an integer");
        match d {
            "L" => {
                for _ in 0..r {
                    head.col -= 1;
                    catch(&head, &mut tail);
                    trail.insert(tail);
                }
            }
            "R" => {
                for _ in 0..r {
                    head.col += 1;
                    catch(&head, &mut tail);
                    trail.insert(tail);
                }
            }
            "U" => {
                for _ in 0..r {
                    head.row -= 1;
                    catch(&head, &mut tail);
                    trail.insert(tail);
                }
            }
            "D" => {
                for _ in 0..r {
                    head.row += 1;
                    catch(&head, &mut tail);
                    trail.insert(tail);
                }
            }
            _ => {
                panic!("Unexpected head movement {d}");
            }
        }
    }
    let count = trail.len();
    println!("{count} distinct positions of the tail");
}

fn chase(rope: &mut [Position; 10]) {
    for knot in 0..9 {
        let head = rope[knot];
        let mut tail = rope[knot + 1];
        catch(&head, &mut tail);
        rope[knot + 1] = tail;
    }
}

pub fn b() {
    let ctxt = readfile("09");

    let mut knots: [Position; 10] = [Default::default(); 10];

    let mut trail = HashSet::<Position>::new();
    trail.insert(knots[9]);

    for line in ctxt.lines() {
        let (d, r) = line.split_once(' ').unwrap();
        let r: u32 = r.parse().expect("The right hand side should be an integer");
        match d {
            "L" => {
                for _ in 0..r {
                    knots[0].col -= 1;
                    chase(&mut knots);
                    trail.insert(knots[9]);
                }
            }
            "R" => {
                for _ in 0..r {
                    knots[0].col += 1;
                    chase(&mut knots);
                    trail.insert(knots[9]);
                }
            }
            "U" => {
                for _ in 0..r {
                    knots[0].row -= 1;
                    chase(&mut knots);
                    trail.insert(knots[9]);
                }
            }
            "D" => {
                for _ in 0..r {
                    knots[0].row += 1;
                    chase(&mut knots);
                    trail.insert(knots[9]);
                }
            }
            _ => {
                panic!("Unexpected head movement {d}");
            }
        }
    }
    let count = trail.len();
    println!("{count} distinct positions of the tail");
}
