# MILP Batch Scheduling Optimization

A Mixed Integer Linear Programming (MILP) solution for batch scheduling problems in manufacturing environments, implemented in Rust using the `good_lp` optimization library with HiGHS solver.

## Overview

This project solves the batch scheduling problem where multiple jobs need to be assigned to batches on a single machine, subject to various constraints including:

- Batch capacity limitations
- Job release dates and due dates
- Processing time requirements
- Batch ordering constraints
- Tardiness minimization

The objective is to minimize the total weighted tardiness across all jobs.

## Problem Formulation

### Variables

- **Binary variables (x)**: Job-to-batch assignments
- **Continuous variables**:
  - Batch release times, processing times, and completion times
  - Job completion times and tardiness values

### Constraints

1. **Assignment constraint**: Each job must be assigned to exactly one batch
2. **Capacity constraint**: Total size of jobs in a batch cannot exceed batch capacity
3. **Release time constraint**: Batch release time must be at least the maximum release time of assigned jobs
4. **Ordering constraint**: Batches must be processed sequentially
5. **Processing time constraint**: Batch processing time must accommodate all assigned jobs
6. **Completion time constraint**: Batch completion time = release time + processing time
7. **Job completion constraint**: Job completion time equals its batch completion time
8. **Tardiness constraint**: Tardiness is the positive difference between completion time and due date

## Project Structure

```
milp_ds_sbo/
├── src/
│   ├── main.rs              # Main optimization logic and solver interface
│   ├── lib.rs               # Library entry point
│   ├── structure.rs         # Job data structure definition
│   └── resources/
│       ├── mod.rs           # Resource module
│       └── problem.rs       # Problem data and test cases
├── Cargo.toml               # Project dependencies and metadata
└── README.md                # This file
```

## Dependencies

- **good_lp**: Linear programming interface for Rust
- **highs**: High-performance linear programming solver

## Installation

1. **Prerequisites**: 
   - Rust (latest stable version)
   - CMake (for building HiGHS solver)

2. **Clone the repository**:
   ```bash
   git clone https://github.com/codernotfound-219/MILP-DS_SBO.git
   cd MILP-DS_SBO
   ```

3. **Build the project**:
   ```bash
   cargo build --release
   ```

## Usage

### Running the Default Problem

```bash
cargo run
```

This will solve the predefined 10-job batch scheduling problem with the following output:
- Objective value (total weighted tardiness)
- Job assignments to batches
- Detailed job information (release dates, processing times, due dates, etc.)
- Batch utilization and timing information

### Example Output

```
Setting up batch scheduling problem...
Number of jobs: 10
Batch capacity: 20

Solving the optimization problem...

=== SOLUTION ===
Objective value (total weighted tardiness): 42.00

Job assignments:
Job 1 (J1) assigned to Batch 1
  - Release Date: 1
  - Processing Time: 9
  - Due Date: 29
  - Size: 9
  - Completion Time: 10.00
  - Tardiness: 0.00
  - Lateness Penalty: 2

Batch details:
Batch 1:
  - Release Time: 1.00
  - Processing Time: 9.00
  - Completion Time: 10.00
  - Total Size: 9.00/20.00
```

### Customizing Problems

To define your own batch scheduling problem:

1. **Modify job data** in `src/resources/problem.rs`:
   ```rust
   pub fn problem1() -> Vec<Job> {
       vec![
           Job {
               release_date: 1,
               processing_time: 9,
               due_date: 29,
               size: 9,
               lateness_penalty: 2,
           },
           // Add more jobs...
       ]
   }
   ```

2. **Adjust capacity** in the same file:
   ```rust
   pub static PROBLEM1_CAPACITY: u32 = 20;
   ```

## Algorithm Details

The solution uses a Mixed Integer Linear Programming approach:

1. **Model formulation**: Converts the scheduling problem into mathematical constraints
2. **Solver integration**: Uses HiGHS solver through the good_lp interface
3. **Big-M constraints**: Handles conditional constraints for job-batch relationships
4. **Sequential processing**: Ensures batches are processed in order

## Performance Considerations

- **Scalability**: The model creates O(n²) binary variables for n jobs, so performance may degrade for very large problems
- **Big-M value**: Set to 1000.0 for numerical stability; may need adjustment for different problem scales
- **Solver choice**: HiGHS is chosen for its performance on MILP problems

## Authors

- Academic research project
- Repository: [MILP-DS_SBO](https://github.com/codernotfound-219/MILP-DS_SBO)

## References

- [good_lp documentation](https://docs.rs/good_lp/)
- [HiGHS solver](https://highs.dev/)
- Mixed Integer Linear Programming for batch scheduling problems
