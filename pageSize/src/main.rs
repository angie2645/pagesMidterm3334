
use std::fs;

fn calculate_penalty(page_size: u64, workloads: &Vec<Vec<u64>>) -> f64 {
    let mut total_penalty: u64 = 0;

    for day in workloads {
        let mut total_pages: u64 = 0;
        let mut total_unused: u64 = 0;

        for &memory in day {
            let pages_needed = memory.div_ceil(page_size);
            let allocated_memory = pages_needed * page_size;
            let unused_memory = allocated_memory - memory;

            total_pages += pages_needed;
            total_unused += unused_memory;
        }

        let bookkeeping = total_pages * 64;
        let daily_penalty = total_unused + bookkeeping;

        total_penalty += daily_penalty;
    }

    total_penalty as f64 / workloads.len() as f64
}

fn main() {
    let mut workloads: Vec<Vec<u64>> = Vec::new();
    let mut total_memory: u64 = 0;
    let mut total_requests: u64 = 0;

    for day in 0..32 {
        let path = format!(
            "/workspaces/pagesMidterm3334/pageSize/src/workloads/loads ({})",
            day
        );

        let contents = fs::read_to_string(&path)
            .expect("Could not read file");

        let mut requests: Vec<u64> = Vec::new();

        for number in contents.split_whitespace() {
            let memory: u64 = number.parse().expect("Invalid number");

            requests.push(memory);
            total_memory += memory;
            total_requests += 1;
        }

        assert_eq!(requests.len(), 128);

        workloads.push(requests);
    }

    let overall_average =
        (total_memory as f64 / total_requests as f64).ceil() as u64;

    let mut best_page_size: u64 = 0;
    let mut best_penalty = f64::MAX;

    for page_size in 30000..=33050 {
        let penalty = calculate_penalty(page_size, &workloads);

        if penalty < best_penalty {
            best_penalty = penalty;
            best_page_size = page_size;
        }
    }

    println!("Overall average memory: {} bits", overall_average);
    println!("Best page size: {} bits", best_page_size);
    println!("Average daily memory penalty: {} bits", best_penalty.ceil() as u64);
}
