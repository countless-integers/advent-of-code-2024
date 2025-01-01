pub fn run<I>(_input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
   0 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input_data = vec![
            "MMMSXXMASM".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 0);
    }
}