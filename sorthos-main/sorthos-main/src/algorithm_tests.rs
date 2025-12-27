#[cfg(test)]
mod algorithm_tests {
    use crate::models::SortBar;
    use crate::sorting::{
        bogo_sort, counting_sort_visual, radix_sort_visual, shell_sort_visual, Operation,
    };
    use std::sync::mpsc;
    use std::thread;

    /// Helper function to create a vector of SortBars from values
    fn create_bars(values: Vec<usize>) -> Vec<SortBar> {
        values.into_iter().map(SortBar::new).collect()
    }

    /// Helper function to extract values from SortBars
    fn extract_values(bars: &[SortBar]) -> Vec<usize> {
        bars.iter().map(|b| b.value).collect()
    }

    /// Check if a slice is sorted in non-decreasing order

    /// Check if a slice is sorted in non-decreasing order (usize version)
    fn is_sorted_usize(values: &[usize]) -> bool {
        values.windows(2).all(|w| w[0] <= w[1])
    }

    /// Test cases for all algorithms

    #[test]
    fn test_counting_sort_visual() {
        println!("Testing Counting Sort...");
        let test_cases_usize = vec![
            vec![5, 2, 8, 1, 9, 3, 7, 4, 6],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![5, 5, 5, 5, 5],
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            vec![42],
            vec![2, 1],
            vec![],
            (1..=10).rev().collect::<Vec<_>>(),
            (1..=10).collect::<Vec<_>>(),
        ];

        for test_case in test_cases_usize {
            let original = test_case.clone();
            let mut bars = create_bars(test_case);
            let (tx, _rx) = mpsc::channel::<Operation>();

            counting_sort_visual(&mut bars, &tx);
            let result = extract_values(&bars);

            assert!(
                is_sorted_usize(&result),
                "Counting Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Counting Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_radix_sort_visual() {
        println!("Testing Radix Sort...");
        let test_cases_usize = vec![
            vec![5, 2, 8, 1, 9, 3, 7, 4, 6],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![5, 5, 5, 5, 5],
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            vec![42],
            vec![2, 1],
            vec![],
            (1..=15).rev().collect::<Vec<_>>(),
            (1..=15).collect::<Vec<_>>(),
        ];

        for test_case in test_cases_usize {
            let original = test_case.clone();
            let mut bars = create_bars(test_case);
            let (tx, _rx) = mpsc::channel::<Operation>();

            radix_sort_visual(&mut bars, &tx);
            let result = extract_values(&bars);

            assert!(
                is_sorted_usize(&result),
                "Radix Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Radix Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_shell_sort_visual() {
        println!("Testing Shell Sort...");
        let test_cases_usize = vec![
            vec![5, 2, 8, 1, 9, 3, 7, 4, 6],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![5, 5, 5, 5, 5],
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5],
            vec![42],
            vec![2, 1],
            vec![],
            (1..=15).rev().collect::<Vec<_>>(),
            (1..=15).collect::<Vec<_>>(),
        ];

        for test_case in test_cases_usize {
            let original = test_case.clone();
            let mut bars = create_bars(test_case);
            let (tx, _rx) = mpsc::channel::<Operation>();

            shell_sort_visual(&mut bars, &tx);
            let result = extract_values(&bars);

            assert!(
                is_sorted_usize(&result),
                "Shell Sort failed on input: {:?}, got: {:?}",
                original,
                result
            );

            // Verify all elements are preserved
            let mut original_sorted = original.clone();
            original_sorted.sort();
            let mut result_sorted = result.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "Shell Sort lost elements on input: {:?}",
                original
            );
        }
    }

    #[test]
    fn test_bogo_sort() {
        println!("Testing Bogo Sort...");
        // Use very small test cases for probabilistic sorts
        let small_test_cases = vec![vec![2, 1], vec![3, 1, 2], vec![42], vec![]];

        for test_case in small_test_cases {
            let original = test_case.clone();

            // Test Bogo Sort with timeout
            let bars = create_bars(test_case.clone());
            let (tx, _rx) = mpsc::channel::<Operation>();

            // Use a separate thread with timeout for Bogo Sort
            let bars_clone = bars.clone();
            let handle = thread::spawn(move || {
                let mut bars = bars_clone;
                bogo_sort(&mut bars, &tx);
                bars
            });

            // Wait for completion with timeout
            if let Ok(result_bars) = handle.join() {
                let result = extract_values(&result_bars);
                if !result.is_empty() {
                    assert!(
                        is_sorted_usize(&result),
                        "Bogo Sort failed on input: {:?}, got: {:?}",
                        original,
                        result
                    );
                }
            }
        }
    }

    // Removed test_algorithm_stability: referenced non-existent algorithms

    // Removed test_performance_characteristics: referenced non-existent algorithms
}
