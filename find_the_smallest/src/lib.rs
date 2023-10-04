/*
You have a positive number n consisting of digits. You can do at most one operation: Choosing the index of a digit in the number, remove this digit at that index and insert it back to another or at the same place in the number in order to find the smallest number you can get.

Task:
Return an array or a tuple or a string depending on the language (see "Sample Tests") with

the smallest number you got
the index i of the digit d you took, i as small as possible
the index j (as small as possible) where you insert this digit d to have the smallest number.
Examples:
smallest(261235) --> [126235, 2, 0] or (126235, 2, 0) or "126235, 2, 0"
126235 is the smallest number gotten by taking 1 at index 2 and putting it at index 0

smallest(209917) --> [29917, 0, 1] or ...

[29917, 1, 0] could be a solution too but index `i` in [29917, 1, 0] is greater than 
index `i` in [29917, 0, 1].
29917 is the smallest number gotten by taking 2 at index 0 and putting it at index 1 which gave 029917 which is the number 29917.

smallest(1000000) --> [1, 0, 6] or ...

*/

fn get_digits(n: i64) -> Vec<i64> {
    let mut digits: Vec<i64> = Vec::new();
    let mut current_n = n;
    loop {
        if current_n <= 1 {
            break;
        }f
        digits.insert(0, current_n % 10);
        current_n /= 10;
    }
    digits
}

fn find_smallest_indexes(digits: &Vec<i64>) -> Result<(usize, usize), ()> {
    for (index, value) in digits.iter().enumerate() {
        let mut current_smallest_value = *value;
        let mut upper_index = index;
        for j in (index+1)..digits.len() {
            if digits[j] < current_smallest_value {
                current_smallest_value = digits[j];
                upper_index = j;
            }
        }
        if current_smallest_value != *value {
            return Ok((upper_index, index));
        }
    }
    // technically should never happen
    return Err(());
}

pub fn smallest(n: i64) -> (i64, usize, usize) {
    let mut digits = get_digits(n);
    let (upper_index, lower_index) =  match find_smallest_indexes(&digits) {
        Ok(res) => res,
        Err(()) => return (n, 0, 0)
    };

    let tmp = digits[upper_index];
    digits.remove(upper_index);
    digits.insert(lower_index, tmp);

    let smallest_number = digits
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    return (smallest_number, upper_index, lower_index);
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn testing(n: i64, exp: (i64, usize, usize)) -> () {
        let ans = smallest(n);
        assert_eq!(ans, exp, "Testing: {}", n);
    }

    #[test]
    fn basic_tests() {
        testing(261235, (126235, 2, 0));
        testing(209917, (29917, 0, 1));
        testing(285365, (238565, 3, 1));    
    }
}