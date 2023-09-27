pub fn reverse(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }

    let reversed_str: String = x.abs().to_string().trim().chars().rev().collect();

    if let Ok(value) = reversed_str.parse::<i32>() {
        if x >= i32::MIN && x <= i32::MAX {
            if x < 0 {
                return value * -1;
            } else {
                return value;
            }
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}

fn main() {
    println!("{}", reverse(123));
    println!("{}", reverse(-123));
    println!("{}", reverse(120));
    println!("{}", reverse(0));
    println!("{}", reverse(1534236469));
}
