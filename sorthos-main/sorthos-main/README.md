# Sorthos


## Features

- **Real-time Visualization**: Watch sorting algorithms work step-by-step with visual feedback
- **Comprehensive Algorithm Collection**: 20+ sorting algorithms from educational to production-grade
- **Interactive GUI**: User-friendly interface with algorithm selection and customization options
- **Performance Analysis**: Compare algorithms by time complexity, stability, and practical performance
- **Educational Value**: Perfect for learning algorithm design patterns and trade-offs
- **Modern Context**: Includes algorithms used in real programming languages and systems
- **Dark/Light Theme**: Toggle between visual themes for comfortable viewing

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd sorthos
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

### Running Tests

Execute the comprehensive test suite:
```bash
cargo test
```

## Implemented Algorithms

### Production-Grade Algorithms

These algorithms are used in real programming languages and systems:

#### **Timsort**
- **File**: `src/Sorting/timsort.rs`
- **Status**: ✅ Functional
- **Stability**: Stable
- **Complexity**: O(n log n) worst case, O(n) best case (adaptive)
- **Used In**: Python 2.3-3.10, Java 7+ (objects), Android
- **Description**: Hybrid merge sort + insertion sort designed for real-world data with existing runs
- **Note**: Replaced by Powersort in Python 3.11 due to improved merge policy



#### **Heapsort**
- **File**: `src/Sorting/heap_sort_visual.rs`
- **Status**: ✅ Functional
- **Stability**: Unstable
- **Complexity**: O(n log n) all cases
- **Used In**: Linux kernel, real-time systems, fallback for quicksort variants
- **Description**: Comparison-based sort using binary heap data structure
- **Advantage**: Guaranteed O(n log n) performance, in-place, used when predictable performance is required

#### **Merge Sort**
- **File**: `src/Sorting/merge_sort_visual.rs`
- **Status**: ✅ Functional
- **Stability**: Stable
- **Complexity**: O(n log n) all cases
- **Used In**: External sorting, linked lists, when stability is required
- **Description**: Divide-and-conquer algorithm invented by John von Neumann (1945)
- **Advantage**: Stable, predictable performance, excellent for linked lists and external sorting

#### **Quicksort**
- **File**: `src/Sorting/quicksort_visual.rs`
- **Status**: ✅ Functional
- **Stability**: Unstable
- **Complexity**: O(n log n) average, O(n²) worst case
- **Used In**: Many standard libraries with optimizations, basis for hybrid algorithms
- **Description**: Divide-and-conquer algorithm developed by Tony Hoare (1959)
- **Note**: Modern implementations use median-of-three pivoting and fall back to heapsort for worst cases

### Efficient Specialized Algorithms

#### **Counting Sort**
- **File**: `src/Sorting/counting_sort_visual.rs`
- **Status**: ✅ Functional (with safety checks)
- **Stability**: Stable
- **Complexity**: O(n + k) where k is the range of values
- **Used In**: Radix sort subroutine, small integer ranges
- **Description**: Non-comparison sort that counts occurrences of each value
- **Limitation**: Only suitable when k is not significantly larger than n

#### **Radix Sort**
- **File**: `src/Sorting/radix_sort_visual.rs`
- **Status**: ✅ Functional
- **Stability**: Stable
- **Complexity**: O(d × (n + k)) where d is number of digits
- **Used In**: Integer sorting, digital systems
- **Description**: Non-comparison sort processing digits from least to most significant
- **Advantage**: Can achieve linear time for fixed-width integers

#### **Shell Sort**
- **File**: `src/Sorting/shell_sort_visual.rs`
- **Status**: ✅ Functional
- **Stability**: Unstable
- **Complexity**: O(n^1.25) to O(n^1.5) depending on gap sequence
- **Description**: Generalization of insertion sort using decreasing gap sequences
- **Advantage**: Simple implementation, better than O(n²) algorithms for medium-sized arrays

### Advanced Research Algorithms





### Educational Algorithms

These algorithms are primarily used for teaching algorithmic concepts:

#### **Bubble Sort**
- **File**: `src/Sorting/bubble_sort.rs`
- **Status**: ✅ Functional
- **Stability**: Stable
- **Complexity**: O(n²) average and worst case, O(n) best case (adaptive)
- **Description**: Named for how larger elements "bubble" to the top
- **Educational Value**: Simple to understand, demonstrates comparison-based sorting
- **Note**: Called "the generic bad algorithm" in computer science education

#### **Insertion Sort**
- **File**: `src/Sorting/insertion_sort.rs`
- **Status**: ✅ Functional
- **Stability**: Stable
- **Complexity**: O(n²) average and worst case, O(n) best case (adaptive)
- **Description**: Builds sorted array one element at a time, like sorting playing cards
- **Practical Use**: Efficient for small arrays, used as subroutine in hybrid algorithms
- **Advantage**: Simple, adaptive, stable, in-place, online

#### **Selection Sort**
- **File**: `src/Sorting/selection_sort.rs`
- **Status**: ✅ Functional
- **Stability**: Unstable (stable variants exist)
- **Complexity**: O(n²) all cases
- **Description**: Finds minimum element and swaps it to the beginning
- **Advantage**: Minimizes number of swaps, useful when writing to memory is expensive

#### **Cocktail Sort (Bidirectional Bubble Sort)**
- **File**: `src/Sorting/cocktail_sort_visual.rs`
- **Status**: ✅ Functional
- **Stability**: Stable
- **Complexity**: O(n²) average and worst case
- **Description**: Variation of bubble sort that sorts in both directions alternately
- **Advantage**: Slightly better performance than bubble sort on some inputs

#### **Gnome Sort**
- **File**: `src/Sorting/gnome_sort_visual.rs`
- **Status**: ✅ Functional
- **Stability**: Stable
- **Complexity**: O(n²) worst case, O(n) best case
- **Description**: Simple algorithm similar to insertion sort
- **Educational Value**: Demonstrates position-based sorting approach

### Specialized and Novelty Algorithms



#### **Bogo Sort**
- **File**: `src/core/sorting.rs`
- **Status**: ✅ Functional (small arrays only)
- **Stability**: Not guaranteed
- **Complexity**: O((n+1)!) worst case, O(n) best case
- **Description**: Randomly shuffles array until sorted
- **Educational Value**: Demonstrates the importance of algorithmic design vs. random approaches



## Algorithm Categories by Practical Usage

### **High-Performance Production Algorithms**
- **Introsort**: C++ standard library default
- **Timsort**: Python (legacy), Java objects, Android
- **Merge Sort**: External sorting, stable sorting requirements
- **Heapsort**: Real-time systems, Linux kernel

### **Specialized Use Cases**
- **Counting Sort**: Small integer ranges, histograms
- **Radix Sort**: Fixed-width integers, digital systems
- **Shell Sort**: Medium arrays, simple implementation needed

### **Educational and Research**
- **Bubble/Insertion/Selection Sort**: Algorithm courses, small datasets
- **Quicksort**: Teaching divide-and-conquer, basis for hybrid algorithms
- **Stooge/Slow/Bogo Sort**: Demonstrating algorithmic complexity

### **Research and Experimental**


## Performance and Stability Analysis

### **Stability** (preserves relative order of equal elements)
- **Stable**: Merge Sort, Timsort, Bubble Sort, Insertion Sort, Counting Sort, Radix Sort
- **Unstable**: Quicksort, Heapsort, Selection Sort, Shell Sort
- **Stability Matters For**: Multi-key sorting, maintaining original order of tied elements

### **Adaptive Performance** (faster on partially sorted data)
- **Adaptive**: Timsort, Insertion Sort, Bubble Sort
- **Non-Adaptive**: Selection Sort, Heapsort, Merge Sort (standard)
- **Adaptive Advantage**: Real-world data often has existing order

### **Memory Usage**
- **In-Place O(1)**: Bubble, Selection, Insertion, Heapsort, Shell Sort
- **O(log n) auxiliary**: Quicksort (recursion stack), Introsort
- **O(n) auxiliary**: Merge Sort, Timsort, Counting Sort

### **Real-World Performance Factors**
1. **Cache Locality**: Quicksort > Heapsort > Merge Sort
2. **Branch Prediction**: Counting Sort > Comparison-based sorts
3. **Instruction Count**: Simple algorithms may outperform complex ones for small arrays
4. **Data Characteristics**: Adaptive algorithms excel on real-world data

## Modern Usage

### **Language Implementation Choices**
- **Python**: Timsort (2.3-3.10) → Powersort (3.11+)
- **Java**: Timsort for objects, dual-pivot quicksort for primitives
- **C++**: Introsort in most standard library implementations
- **JavaScript V8**: Timsort

## Testing and Quality Assurance

### **Comprehensive Test Coverage**
- **29+ test cases** covering all implemented algorithms
- **Multiple scenarios**: Random, sorted, reverse-sorted, duplicates, edge cases
- **Stability verification**: Ensuring stable sorts preserve element order
- **Performance validation**: Complexity verification for different input sizes
- **Edge case handling**: Empty arrays, single elements, all equal values

### **Test Categories**
```bash
# Run all tests
cargo test

# Run specific algorithm tests
cargo test test_timsort_functionality

# Run edge case tests
cargo test test_sorting_edge_cases

# Run with detailed output
cargo test -- --nocapture
```

## Project Structure

```
sorthos/
├── src/
│   ├── Sorting/           # Individual algorithm implementations
│   │   ├── timsort.rs     # Production-grade algorithms
│   │   ├── counting_sort_visual.rs
│   │   ├── radix_sort_visual.rs
│   │   ├── bubble_sort.rs  # Educational algorithms
│   │   ├── insertion_sort.rs
│   │   └── ...
│   ├── core/              # Core sorting logic and simple algorithms
│   ├── gui/               # User interface components
│   ├── assets/            # Visual assets and resources
│   ├── algorithm_tests.rs # Algorithm-specific tests
│   └── tests.rs           # Comprehensive test suite
├── target/                # Compiled binaries and dependencies
├── Cargo.toml             # Project configuration
└── README.md              # This documentation
```


## Contributing

Contributions welcome!
- Additional modern sorting algorithms
- Performance optimizations
- Visualization improvements

- Parallel sorting algorithms

## License

BSD-3-Clause

## Dependencies

- **egui**: Modern immediate mode GUI framework
- **eframe**: Application framework for egui
- **Standard Library**: Comprehensive use of Rust's std collections and algorithms
