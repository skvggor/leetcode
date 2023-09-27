pub fn is_valid(s: String) -> bool {
    let mut results: Vec<bool> = Vec::new();

    for item in s.chars().collect::<Vec<char>>().windows(2) {
        let current = item[0];
        let next = item[1];

        println!("Current {:?}, Next {:?}", current, next);

        // if current.to_string() == "(" && next.to_string() == ")" {
        //     results.push(true);
        // }

        if current.to_string() == "(" && next.to_string() != ")" {
            results.push(false);
        }

        // if current.to_string() == "{" && next.to_string() == "}" {
        //     results.push(true);
        // }

        if current.to_string() == "{" && next.to_string() != "}" {
            results.push(false);
        }

        // if current.to_string() == "[" && next.to_string() == "]" {
        //     results.push(true);
        // }

        if current.to_string() == "[" && next.to_string() != "]" {
            results.push(false);
        }
    }

    println!("RESULT {:?} {:?}", results, results.len());

    results.iter().all(|&x| x)
    //     true
    // } else {
    //     false
    // }
}

fn main() {
    println!("FINAL {}", is_valid("()[]{}{}{}{".to_string()));
}

/*

1 3 5
2 4 6

*/
