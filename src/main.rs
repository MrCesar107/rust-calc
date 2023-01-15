use regex::Regex;

fn main() {
    // Regular expresions
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_sus = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_prod = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    // let re_div = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    // User data
    println!("Please enter the math expression: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // Products
    loop {
        // Apply math operations
        let caps = re_prod.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let product = left_value * right_value;

        expression = expression.replace(cap_expression, &product.to_string());
    }

    // Additions
    loop {
        // Apply math operations
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let addition = left_value + right_value;

        expression = expression.replace(cap_expression, &addition.to_string());
    }

    // Substractions
    loop {
        // Apply math operations
        let caps = re_sus.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let substraction = left_value - right_value;

        expression = expression.replace(cap_expression, &substraction.to_string());
    }


    // Show results
    println!("The result is: {expression}");
}
