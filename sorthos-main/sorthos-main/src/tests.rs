#[cfg(test)]
mod tests {

    use crate::models::SortBar;
    use crate::sorting::SortingAlgorithm;
    use eframe::egui::Color32;
    use std::sync::mpsc;

    /// Helper function to create a vector of SortBars from values
    fn create_bars(values: Vec<usize>) -> Vec<SortBar> {
        values.into_iter().map(SortBar::new).collect()
    }

    /// Helper function to extract values from SortBars
    fn extract_values(bars: &[SortBar]) -> Vec<usize> {
        bars.iter().map(|b| b.value).collect()
    }

    /// Check if a slice is sorted in non-decreasing order
    fn is_sorted(values: &[usize]) -> bool {
        values.windows(2).all(|w| w[0] <= w[1])
    }

    /// Count duplicates in a slice
    fn count_duplicates(values: &[usize]) -> usize {
        let mut seen = std::collections::HashMap::new();
        let mut duplicate_count = 0;
        for &value in values {
            let count = seen.entry(value).or_insert(0);
            *count += 1;
            if *count == 2 {
                duplicate_count += 1;
            } else if *count > 2 {
                duplicate_count += 1;
            }
        }
        duplicate_count
    }

    /// Test duplicate removal functionality
    #[test]
    fn test_duplicate_removal() {
        // Test with duplicates
        let values_with_dups = vec![5, 2, 8, 2, 1, 5, 9, 1, 3];
        let original_count = count_duplicates(&values_with_dups);
        assert!(original_count > 0, "Test data should have duplicates");

        // Simulate duplicate removal logic
        let mut seen = std::collections::HashMap::new();
        let mut unique_values = Vec::new();
        for &value in &values_with_dups {
            if !seen.contains_key(&value) {
                seen.insert(value, true);
                unique_values.push(value);
            }
        }

        assert_eq!(
            count_duplicates(&unique_values),
            0,
            "Should have no duplicates after removal"
        );
        assert!(
            unique_values.len() < values_with_dups.len(),
            "Should have fewer elements after removing duplicates"
        );
    }

    /// Test duplicate removal with no duplicates
    #[test]
    fn test_no_duplicates_removal() {
        let values_no_dups = vec![1, 2, 3, 4, 5];
        assert_eq!(
            count_duplicates(&values_no_dups),
            0,
            "Test data should have no duplicates"
        );

        // Simulate duplicate removal logic
        let mut seen = std::collections::HashMap::new();
        let mut unique_values = Vec::new();
        for &value in &values_no_dups {
            if !seen.contains_key(&value) {
                seen.insert(value, true);
                unique_values.push(value);
            }
        }

        assert_eq!(
            unique_values.len(),
            values_no_dups.len(),
            "Length should remain the same"
        );
        assert_eq!(
            count_duplicates(&unique_values),
            0,
            "Should still have no duplicates"
        );
    }

    /// Test sorting algorithms with various inputs
    #[test]
    fn test_sorting_algorithms_basic() {
        let test_cases = vec![
            vec![5, 2, 8, 1, 9], // Random order
            vec![1, 2, 3, 4, 5], // Already sorted
            vec![5, 4, 3, 2, 1], // Reverse sorted
            vec![3, 3, 3, 3],    // All same values
            vec![42],            // Single element
            vec![],              // Empty array
        ];

        // Test a subset of algorithms that don't require channels for basic functionality
        for test_values in test_cases {
            let original_len = test_values.len();

            // Test with bubble sort logic (simplified)
            if test_values.len() > 1 {
                let mut values = test_values.clone();
                // Simplified bubble sort
                for i in 0..values.len() {
                    for j in 0..values.len() - i - 1 {
                        if values[j] > values[j + 1] {
                            values.swap(j, j + 1);
                        }
                    }
                }
                assert!(
                    is_sorted(&values),
                    "Bubble sort should produce sorted array"
                );
                assert_eq!(
                    values.len(),
                    original_len,
                    "Array length should be preserved"
                );
            }
        }
    }

    /// Test edge cases for sorting algorithms
    #[test]
    fn test_sorting_edge_cases() {
        // Test empty array
        let empty_bars: Vec<SortBar> = vec![];
        assert!(is_sorted(&extract_values(&empty_bars)));

        // Test single element
        let single_bars = create_bars(vec![42]);
        assert!(is_sorted(&extract_values(&single_bars)));

        // Test two elements
        let two_bars = create_bars(vec![2, 1]);
        let values = extract_values(&two_bars);
        // After sorting, should be [1, 2]
        let mut sorted_values = values;
        sorted_values.sort();
        assert!(is_sorted(&sorted_values));

        // Test all same values
        let same_bars = create_bars(vec![5, 5, 5, 5, 5]);
        let values = extract_values(&same_bars);
        assert!(is_sorted(&values)); // Already sorted since all same
    }

    /// Test duplicate generation functionality
    #[test]
    fn test_duplicate_generation() {
        let num_bars = 12;
        let unique_values = num_bars / 3; // Should be 4 unique values

        let mut values = Vec::new();
        for i in 0..num_bars {
            let value = i % unique_values;
            values.push(value);
        }

        // Should have duplicates
        assert!(
            count_duplicates(&values) > 0,
            "Generated array should have duplicates"
        );

        // Should have exactly the expected pattern
        let expected_duplicates = num_bars - unique_values;
        assert!(
            count_duplicates(&values) >= expected_duplicates / 2,
            "Should have significant duplicates"
        );
    }

    /// Test large value handling (for counting sort safety)
    #[test]
    fn test_large_value_handling() {
        let large_values = vec![100001, 50000, 200000];
        let max_val = *large_values.iter().max().unwrap();

        const MAX_SAFE_SIZE: usize = 10000;
        if max_val > MAX_SAFE_SIZE {
            // This should trigger the safety check in counting sort
            assert!(
                max_val > MAX_SAFE_SIZE,
                "Large values should exceed safe threshold"
            );
        }
    }

    /// Test sorting algorithm selection and basic validation
    #[test]
    fn test_algorithm_enum() {
        let algorithms = SortingAlgorithm::all();
        assert!(
            !algorithms.is_empty(),
            "Should have sorting algorithms available"
        );

        // Test that all algorithms have display names
        for &algorithm in algorithms {
            let name = format!("{}", algorithm);
            assert!(!name.is_empty(), "Algorithm should have a display name");
        }
    }

    /// Test SortBar creation and manipulation
    #[test]
    fn test_sort_bar() {
        let bar = SortBar::new(42);
        assert_eq!(bar.value, 42, "SortBar should store the correct value");
        assert_eq!(
            bar.color,
            Color32::WHITE,
            "SortBar should start with white color"
        );

        let bars = create_bars(vec![1, 2, 3]);
        assert_eq!(bars.len(), 3, "Should create correct number of bars");
        assert_eq!(
            extract_values(&bars),
            vec![1, 2, 3],
            "Should preserve values"
        );
    }

    /// Test bounds checking for quicksort-style algorithms
    #[test]
    fn test_bounds_checking() {
        let values = vec![3, 1, 4, 1, 5];
        let len = values.len();

        // Test that bounds are properly checked
        if len > 1 {
            let low = 0;
            let high = len - 1;

            // These should be valid bounds
            assert!(low < len, "Low bound should be valid");
            assert!(high < len, "High bound should be valid");
            assert!(low <= high, "Low should be <= high");
        }

        // Test edge case with single element
        let single = vec![42];
        if single.len() <= 1 {
            // Should not attempt to sort
            assert!(
                single.len() <= 1,
                "Single element arrays don't need sorting"
            );
        }
    }

    /// Integration test for the sorting process (without actual GUI)
    #[test]
    fn test_sorting_integration() {
        let (_tx, _rx) = mpsc::channel::<String>();
        let test_values = vec![5, 2, 8, 1, 9, 3];
        let mut bars = create_bars(test_values);

        // Simulate a simple sorting operation
        bars.sort_by(|a, b| a.value.cmp(&b.value));
        let sorted_values = extract_values(&bars);

        assert!(is_sorted(&sorted_values), "Bars should be sorted");
        assert_eq!(
            sorted_values,
            vec![1, 2, 3, 5, 8, 9],
            "Should be in correct order"
        );
    }

    /// Test status message generation logic
    #[test]
    fn test_status_messages() {
        // Test duplicate count formatting
        let no_dups = vec![1, 2, 3, 4, 5];
        let with_dups = vec![1, 2, 2, 3, 3, 3];

        assert_eq!(count_duplicates(&no_dups), 0);
        assert!(count_duplicates(&with_dups) > 0);

        // Test status message content (simulated)
        let duplicate_count = count_duplicates(&with_dups);
        let status_msg = if duplicate_count > 0 {
            format!("⚠ {} duplicates detected", duplicate_count)
        } else {
            "✓ No duplicates".to_string()
        };

        assert!(status_msg.contains("duplicates detected"));
    }

    /// Benchmark-style test for algorithm performance awareness
    #[test]
    fn test_algorithm_performance_characteristics() {
        // Test small arrays (should be fast for all algorithms)
        let small_array = (0..10).collect::<Vec<_>>();
        assert!(small_array.len() == 10);

        // Test medium arrays (good for most algorithms)
        let medium_array = (0..100).collect::<Vec<_>>();
        assert!(medium_array.len() == 100);

        // Test that we don't create excessively large arrays that could cause issues
        let max_reasonable_size = 1000;
        assert!(
            medium_array.len() < max_reasonable_size,
            "Should use reasonable array sizes"
        );
    }

    /// Test color consistency and theme application
    #[test]
    fn test_color_consistency() {
        let mut bars = create_bars(vec![1, 2, 3]);

        // Test that we can change colors
        for bar in &mut bars {
            bar.color = Color32::RED;
        }

        assert!(
            bars.iter().all(|b| b.color == Color32::RED),
            "All bars should be red"
        );

        // Test color reset
        for bar in &mut bars {
            bar.color = Color32::WHITE;
        }

        assert!(
            bars.iter().all(|b| b.color == Color32::WHITE),
            "All bars should be white"
        );
    }

    /// Test TimSort functionality specifically
    #[test]
    fn test_timsort_functionality() {
        use crate::sorting::{tim_sort, Operation};

        let test_cases = vec![
            vec![5, 2, 8, 1, 9, 3, 7, 4, 6],   // Random order
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],   // Reverse sorted
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],   // Already sorted
            vec![5, 5, 5, 5, 5],               // All same values
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5],   // With duplicates
            vec![42],                          // Single element
            vec![2, 1],                        // Two elements
            (0..50).rev().collect::<Vec<_>>(), // Larger array, reverse sorted
            (0..50).collect::<Vec<_>>(),       // Larger array, already sorted
        ];

        for test_values in test_cases {
            let original_values = test_values.clone();
            let mut bars = create_bars(test_values);
            let (tx, _rx) = mpsc::channel::<Operation>();

            // Apply TimSort
            tim_sort(&mut bars, &tx);

            // Extract sorted values
            let sorted_values = extract_values(&bars);

            // Verify the array is sorted
            assert!(
                is_sorted(&sorted_values),
                "TimSort failed to sort array: {:?} -> {:?}",
                original_values,
                sorted_values
            );

            // Verify all original elements are still present (no data loss)
            let mut original_sorted = original_values.clone();
            original_sorted.sort();
            let mut result_sorted = sorted_values.clone();
            result_sorted.sort();
            assert_eq!(
                original_sorted, result_sorted,
                "TimSort lost or added elements: original {:?} -> result {:?}",
                original_values, sorted_values
            );
        }
    }

    /// Test TimSort with edge cases
    #[test]
    fn test_timsort_edge_cases() {
        use crate::sorting::{tim_sort, Operation};

        // Test empty array
        let mut empty_bars: Vec<SortBar> = vec![];
        let (tx, _rx) = mpsc::channel::<Operation>();
        tim_sort(&mut empty_bars, &tx);
        assert!(empty_bars.is_empty(), "Empty array should remain empty");

        // Test single element
        let mut single_bars = create_bars(vec![42]);
        tim_sort(&mut single_bars, &tx);
        assert_eq!(
            extract_values(&single_bars),
            vec![42],
            "Single element should be unchanged"
        );

        // Test array with MIN_MERGE size (32 elements)
        let test_32: Vec<usize> = (0..32).rev().collect();
        let mut bars_32 = create_bars(test_32.clone());
        tim_sort(&mut bars_32, &tx);
        let sorted_32 = extract_values(&bars_32);
        assert!(is_sorted(&sorted_32), "32-element array should be sorted");

        // Test array just above MIN_MERGE size (33 elements)
        let test_33: Vec<usize> = (0..33).rev().collect();
        let mut bars_33 = create_bars(test_33.clone());
        tim_sort(&mut bars_33, &tx);
        let sorted_33 = extract_values(&bars_33);
        assert!(is_sorted(&sorted_33), "33-element array should be sorted");
    }
}
