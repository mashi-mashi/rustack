fn add(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();

    stack.push(lhs + rhs);
}
fn sub(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();

    stack.push(lhs - rhs);
}
fn mul(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();

    stack.push(lhs * rhs);
}
fn div(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();

    stack.push(lhs / rhs);
}
fn main() {
    let mut stack = vec![];
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let words: Vec<_> = line.split(" ").collect();
            println!("Line: {words:?}");

            for word in words {
                if let Ok(parsed) = word.parse::<i32>() {
                    stack.push(parsed);
                } else {
                    match word {
                        "+" => add(&mut stack),
                        "-" => sub(&mut stack),
                        "*" => mul(&mut stack),
                        "/" => div(&mut stack),
                        _ => panic!("{word:?} colud not be parsed."),
                    }
                }
            }
            println!("stack: {stack:?}");
        }
    }
}
