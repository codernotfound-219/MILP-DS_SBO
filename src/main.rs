use good_lp::*;

#[derive(Debug)]
struct Job {
    release_date: f64,
    processing_time: f64,
    due_date: f64,
    size: f64,
    lateness_penalty: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Job data from the table
    let jobs = vec![
        Job { release_date: 1.0, processing_time: 9.0, due_date: 29.0, size: 9.0, lateness_penalty: 2.0 },
        Job { release_date: 16.0, processing_time: 4.0, due_date: 31.0, size: 7.0, lateness_penalty: 6.0 },
        Job { release_date: 20.0, processing_time: 5.0, due_date: 42.0, size: 9.0, lateness_penalty: 2.0 },
        Job { release_date: 14.0, processing_time: 3.0, due_date: 22.0, size: 6.0, lateness_penalty: 10.0 },
        Job { release_date: 5.0, processing_time: 6.0, due_date: 22.0, size: 6.0, lateness_penalty: 3.0 },
        Job { release_date: 4.0, processing_time: 9.0, due_date: 27.0, size: 9.0, lateness_penalty: 6.0 },
        Job { release_date: 13.0, processing_time: 1.0, due_date: 17.0, size: 8.0, lateness_penalty: 9.0 },
        Job { release_date: 13.0, processing_time: 4.0, due_date: 22.0, size: 9.0, lateness_penalty: 3.0 },
        Job { release_date: 18.0, processing_time: 8.0, due_date: 28.0, size: 6.0, lateness_penalty: 3.0 },
        Job { release_date: 6.0, processing_time: 4.0, due_date: 38.0, size: 7.0, lateness_penalty: 5.0 },
    ];

    let num_jobs = jobs.len();
    let num_batches = num_jobs; // Maximum possible batches
    let capacity = 15.0;
    let m = 1000.0; // Big M value (reduced for numerical stability)

    println!("Setting up batch scheduling problem...");
    println!("Number of jobs: {}", num_jobs);
    println!("Batch capacity: {}", capacity);

    // Create variables
    let mut vars = variables!();
    
    // Binary variables for job-batch assignments
    let x = vars.add_vector(variable().binary(), num_jobs * num_batches);
    
    // Continuous variables for batches
    let batch_rt = vars.add_vector(variable().min(0.0), num_batches); // Batch release time
    let batch_pt = vars.add_vector(variable().min(0.0), num_batches); // Batch processing time
    let batch_ct = vars.add_vector(variable().min(0.0), num_batches); // Batch completion time
    
    // Continuous variables for jobs
    let job_ct = vars.add_vector(variable().min(0.0), num_jobs); // Job completion time
    let job_tardiness = vars.add_vector(variable().min(0.0), num_jobs); // Job tardiness

    // Helper functions to get variable indices in the x matrix
    let x_var = |j: usize, b: usize| x[j * num_batches + b];

    // Create the problem with solver
    let mut model = vars.minimise(
        jobs.iter().enumerate()
            .map(|(j, job)| job.lateness_penalty * job_tardiness[j])
            .sum::<Expression>()
    ).using(highs);

    // Add constraints

    // 1. Each job must be assigned to exactly one batch
    for j in 0..num_jobs {
        let constraint: Expression = (0..num_batches)
            .map(|b| x_var(j, b))
            .sum();
        model = model.with(constraint.eq(1.0));
    }

    // 2. Batch capacity constraint
    for b in 0..num_batches {
        let constraint: Expression = (0..num_jobs)
            .map(|j| jobs[j].size * x_var(j, b))
            .sum();
        model = model.with(constraint << capacity);
    }

    // 3. Batch release time must be at least the maximum release time of assigned jobs
    for b in 0..num_batches {
        for j in 0..num_jobs {
            // batch_rt[b] >= release_time[j] * x[j][b]
            let constraint = batch_rt[b] - jobs[j].release_date * x_var(j, b);
            model = model.with(constraint >> 0.0);
        }
    }

    // 4. Batch release time ordering (batch b cannot start before batch b-1 completes)
    for b in 1..num_batches {
        let constraint = batch_rt[b] - batch_ct[b-1];
        model = model.with(constraint >> 0.0);
    }

    // 5. Batch processing time must be at least the processing time of each assigned job
    for b in 0..num_batches {
        for j in 0..num_jobs {
            // batch_pt[b] >= processing_time[j] * x[j][b]
            let constraint = batch_pt[b] - jobs[j].processing_time * x_var(j, b);
            model = model.with(constraint >> 0.0);
        }
    }

    // 6. Batch completion time = batch release time + batch processing time
    for b in 0..num_batches {
        let constraint = batch_ct[b] - batch_rt[b] - batch_pt[b];
        model = model.with(constraint.eq(0.0));
    }

    // 7. Job completion time constraint - if job j is assigned to batch b, then job_ct[j] = batch_ct[b]
    for j in 0..num_jobs {
        for b in 0..num_batches {
            // job_ct[j] >= batch_ct[b] - M*(1-x[j][b])
            let constraint1 = job_ct[j] - batch_ct[b] + m * (1.0 - x_var(j, b));
            model = model.with(constraint1 >> 0.0);
            
            // job_ct[j] <= batch_ct[b] + M*(1-x[j][b])
            let constraint2 = job_ct[j] - batch_ct[b] - m * (1.0 - x_var(j, b));
            model = model.with(constraint2 << 0.0);
        }
    }

    // 8. Tardiness calculation: tardiness[j] >= completion_time[j] - due_date[j]
    for j in 0..num_jobs {
        let constraint = job_tardiness[j] - job_ct[j];
        model = model.with(constraint >> -jobs[j].due_date);
    }

    println!("Solving the optimization problem...");

    // Solve the problem using HiGHS
    let solution = model.solve()?;

    // Print results
    println!("\n=== SOLUTION ===");
    
    let total_weighted_tardiness: f64 = jobs.iter().enumerate()
        .map(|(j, job)| solution.value(job_tardiness[j]) * job.lateness_penalty)
        .sum();
    println!("Objective value (total weighted tardiness): {:.2}", total_weighted_tardiness);

    println!("\nJob assignments:");
    for j in 0..num_jobs {
        for b in 0..num_batches {
            if solution.value(x_var(j, b)) > 0.5 {
                println!("Job {} (J{}) assigned to Batch {}", j+1, j+1, b+1);
                println!("  - Release Date: {}", jobs[j].release_date);
                println!("  - Processing Time: {}", jobs[j].processing_time);
                println!("  - Due Date: {}", jobs[j].due_date);
                println!("  - Size: {}", jobs[j].size);
                println!("  - Completion Time: {:.2}", solution.value(job_ct[j]));
                println!("  - Tardiness: {:.2}", solution.value(job_tardiness[j]));
                println!("  - Lateness Penalty: {}", jobs[j].lateness_penalty);
                println!();
            }
        }
    }

    println!("Batch details:");
    for b in 0..num_batches {
        let batch_has_jobs = (0..num_jobs).any(|j| solution.value(x_var(j, b)) > 0.5);
        if batch_has_jobs {
            println!("Batch {}:", b+1);
            println!("  - Release Time: {:.2}", solution.value(batch_rt[b]));
            println!("  - Processing Time: {:.2}", solution.value(batch_pt[b]));
            println!("  - Completion Time: {:.2}", solution.value(batch_ct[b]));
            
            let total_size: f64 = (0..num_jobs)
                .filter(|&j| solution.value(x_var(j, b)) > 0.5)
                .map(|j| jobs[j].size)
                .sum();
            println!("  - Total Size: {:.2}/{:.2}", total_size, capacity);
            println!();
        }
    }

    Ok(())
}
