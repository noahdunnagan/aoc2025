struct Range {
    start: i64,
    end: i64,
}

fn main() {
    let input = "69810572-69955342,3434061167-3434167492,76756725-76781020,49-147,296131-386620,910523-946587,34308309-34358652,64542-127485,640436-659023,25-45,35313993-35393518,753722181-753795479,1544-9792,256-647,444628-483065,5863911-6054673,6969623908-6969778569,658-1220,12631-63767,670238-830345,1-18,214165106-214245544,3309229-3355697";

    // Parse input into ranges
    let ranges: Vec<Range> = input
        .split(",")
        .map(|s| {
            let parts: Vec<&str> = s.split("-").collect();
            Range {
                start: parts[0].parse().unwrap(),
                end: parts[1].parse().unwrap(),
            }
        })
        .collect();

    // Sam's original logic
    let mut sum: i64 = 0;

    for range in ranges {
        for i in range.start..=range.end {
            let number_str = i.to_string();
            let number_length = number_str.len();
            let half_length = number_length / 2;
            let mut found = false;

            for j in 0..half_length {
                let first_j_characters = &number_str[..j + 1];
                let possible_occurrences = number_length as i64 / (j as i64 + 1);

                if number_length as i64 % (j as i64 + 1) != 0 {
                    continue;
                }

                let real_occurrences = number_str.matches(first_j_characters).count() as i64;

                if real_occurrences == possible_occurrences {
                    sum += i;
                    found = true;
                    break;
                }
            }

            if found {
                continue;
            }
        }
    }

    println!("Total: {}", sum);
}
