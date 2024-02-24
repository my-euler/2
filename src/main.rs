fn main() {
    let mut result = 0;
    let mut previous = 0;
    let mut current = 1;
    loop {
        if current > 4000000 {
            break;
        }
        if current % 2 == 0 {
            result += current
        }
        (previous, current) = (current, previous + current)
    }
    println!("{}", result)
}
