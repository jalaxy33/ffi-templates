
#[cfg(test)]
mod test {
    use rust_to_python::*;


    #[test]
    fn test_sum_as_string() {
        let result = sum_as_string(3, 5).unwrap();
        assert_eq!(result, "8");
    }

    #[test]
    fn test_process_numbers() {
        let input = vec![1.0, 2.0, 3.0];
        let result = process_numbers(input).unwrap();
        assert_eq!(result, vec![3.0, 5.0, 7.0]);
    }

    #[test]
    fn test_calculator() {
        let mut calc = Calculator::new(10.0);
        assert_eq!(calc.get_value(), 10.0);
        calc.add(5.0);
        assert_eq!(calc.get_value(), 15.0);
        calc.reset();
        assert_eq!(calc.get_value(), 0.0);
    }
}
