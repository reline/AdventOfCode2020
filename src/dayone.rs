pub fn partone(expense_report: Vec<i32>) -> i32 {
    for x in &expense_report {
        for y in &expense_report {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_repair() {
        let expense_report = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(partone(expense_report), 514579);
    }
}
