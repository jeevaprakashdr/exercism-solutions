use std::u8;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    println!("{:?}", &minefield);
    
    let mut result:Vec<String> = vec![];

    if minefield.is_empty() {
        return result
    }

    for val in minefield {
        let mut min_field: Vec<u8> = Vec::new();
        for c in val.as_bytes() {
            if c.is_ascii_whitespace() {
                min_field.push(c.to_owned())
            }

            if c.eq("*".as_bytes().first().unwrap()) {
                min_field.push(c.to_owned())
            }
        }

        result.push(String::from_utf8_lossy(&min_field.as_slice()).to_string());
    }

    result
}
