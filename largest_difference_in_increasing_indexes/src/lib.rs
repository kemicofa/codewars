/*
Summary: Write a function which takes an array data of numbers and returns the largest difference in indexes j - i such that data[i] <= data[j]

Long Description:

The largestDifference takes an array of numbers. That array is not sorted. Do not sort it or change the order of the elements in any way, or their values.

Consider all of the pairs of numbers in the array where the first one is less than or equal to the second one.

From these, find a pair where their positions in the array are farthest apart.

Return the difference between the indexes of the two array elements in this pair.


*/

fn largest_difference(data: &[i32]) -> usize {
    let mut largest_difference = 0;
    for (i, value) in data.iter().enumerate() {
        for j in (i + 1)..data.len() {
            if *value <= data[j] {
                let tmp = j - i;
                if tmp > largest_difference {
                    largest_difference = tmp;
                }
            }
        }
    }
    largest_difference
}


#[cfg(test)]
mod tests {
    use super::largest_difference;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(data: &[i32], expected: usize) {
        assert_eq!(largest_difference(data), expected, "{ERR_MSG} with data = {data:?}")
    }

    #[test]
    fn sample_tests() {
        dotest(&[9, 4, 1, 10, 3, 4, 0, -1, -2], 4);
        dotest(&[3, 2, 1], 0);
        dotest(&[1, 2, 3], 2);
        dotest(&[9, 4, 1, 2, 3, 0, -1, -2], 2);
        dotest(&[9, 4, 1, 2, 3, 4, 0, -1, -2], 4);
        dotest(&[78, 88, 64, 94, 17, 91, 57, 69, 38, 62, 13, 17, 35, 15, 20], 10);
        dotest(&[4, 3, 3, 1, 5, 2], 4);
    }
}
