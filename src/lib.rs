
pub fn exec() {
    println!("Hello , {{project-name}}.");
}

pub fn add_one(left: usize) -> usize {
    left + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        common::setup();
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
