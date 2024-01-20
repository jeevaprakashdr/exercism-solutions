pub fn annotate(minefield: &[&str]) -> Vec<String> {
    println!("{:?}", &minefield);
    
    let mut result:Vec<String> = vec![];

    if minefield.is_empty() {
        return result
    }

    for val in minefield {
        if val.is_empty() {
            result.push("".to_string());
        }

        if val.eq(&"   ") {
            result.push("   ".to_string())
        }
    }

    result
}
