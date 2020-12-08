pub fn partone(expense_report: &[i32]) -> i32 {
    return process_expense_report(expense_report, 2, 0, 0);
}

pub fn parttwo(expense_report: &[i32]) -> i32 {
    return process_expense_report(expense_report, 3, 0, 0);
}

fn process_expense_report(
    expense_report: &[i32],
    criteria: i8,
    prev_expense: i32,
    product: i32,
) -> i32 {
    let new_criteria = criteria - 1;
    for (pos, &expense) in expense_report.iter().enumerate() {
        let sum = prev_expense + expense;
        if sum > 2020 {
            continue;
        }
        if new_criteria == 0 {
            if sum == 2020 {
                return product * expense;
            }
        } else if expense_report.len() - 1 > pos {
            let slice = &expense_report[pos + 1..expense_report.len()];
            let new_product;
            if product > 0 {
                new_product = product * expense;
            } else {
                new_product = expense;
            }
            let result = process_expense_report(slice, new_criteria, sum, new_product);
            if result > 0 {
                return result;
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let expense_report = [979, 1721, 366, 299, 675, 1456];
        assert_eq!(partone(&expense_report), 514579);
    }

    #[test]
    fn test_part_two() {
        let expense_report = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(parttwo(&expense_report), 241861950);
    }
}
