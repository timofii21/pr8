fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_the_case() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        for (a, b) in data.iter() {
            let inverted_a = invert_the_case(a.to_string());
            let inverted_b = invert_the_case(b.to_string());

            // Виведення результатів
            println!("Inverting '{}' gives '{}'", a, inverted_a);
            println!("Inverting '{}' gives '{}'", b, inverted_b);

            assert_eq!(inverted_a, b.to_string());
            assert_eq!(inverted_b, a.to_string());
        }
    }
}

fn main() {

    let example = "Hello, Привет!";
    let inverted_example = invert_the_case(example.to_string());
    println!("Original: '{}', Inverted: '{}'", example, inverted_example);
}
