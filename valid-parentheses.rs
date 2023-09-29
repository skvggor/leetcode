// https://leetcode.com/submissions/detail/1061913551/

pub fn is_valid(s: String) -> bool {
    let input = s.chars();
    let mut results: Vec<char> = Vec::new();

    let open_brackets = vec!['[', '{', '('];
    let close_brackets = vec![']', '}', ')'];

    fn pair_checker(close_bracket: char) -> char {
        return match close_bracket {
            ']' => { '[' }
            '}' => { '{' }
            ')' => { '(' }
            _ => { ' ' }
        }
    }

    for bracket in input {
        let curr = bracket;

        if open_brackets.contains(&curr) {
            results.insert(0, curr);
        } else if close_brackets.contains(&curr) {
            if results.len() > 0 {
                if results[0] == pair_checker(curr) {
                    results.remove(0);
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    results.len() == 0
}

fn main() {
    // Tests
    println!("{:?}", is_valid("}".to_string()));
    println!("{:?}", is_valid("()".to_string()));
    println!("{:?}", is_valid("()()[]".to_string()));
    println!("{:?}", is_valid("()([]".to_string()));
}
