use common::prelude::*;

const INPUT: &str = "69810572-69955342,3434061167-3434167492,76756725-76781020,49-147,296131-386620,910523-946587,34308309-34358652,64542-127485,640436-659023,25-45,35313993-35393518,753722181-753795479,1544-9792,256-647,444628-483065,5863911-6054673,6969623908-6969778569,658-1220,12631-63767,670238-830345,1-18,214165106-214245544,3309229-3355697";

/// Returns the number of digits in a number.
/// 1234 -> 4, 50 -> 2, 7 -> 1
fn digit_count(n: i64) -> u32 {
    (n as f64).log10().floor() as u32 + 1
}

/// Checks if a range could contain any even digit numbers.
/// (100, 500) -> false because all 3 digit numbers can't be invalid
/// (50, 1500) -> true because it contains 2 and 4 digit numbers
fn has_even_digit_number(start: i64, end: i64) -> bool {
    let start_digits = digit_count(start);
    let end_digits = digit_count(end);

    // start=50 has 2 digits (even), so yes
    if start_digits % 2 == 0 {
        return true;
    }

    // end=1500 has 4 digits (even), so yes
    if end_digits % 2 == 0 {
        return true;
    }

    // 5 to 500 spans 1 digit to 3 digit, crossing through 2 digit territory
    if end_digits - start_digits >= 2 {
        return true;
    }

    false
}

fn part1(input: &str) -> i64 {
    let ranges: Vec<(i64, i64)> = input
        .split(",")
        .map(|item| {
            let parts: Vec<&str> = item.split("-").collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    // Toss out ranges that can't have invalid IDs (all odd digit numbers)
    let cleaned_ranges: Vec<(i64, i64)> = ranges
        .iter()
        .filter(|(start, end)| has_even_digit_number(*start, *end))
        .cloned()
        .collect();

    let mut total: i64 = 0;

    // Invalid IDs come in "families" by digit count:
    //   2 digit: 11, 22, 33... 99         (base 1 to 9 times 11)
    //   4 digit: 1010, 1111... 9999       (base 10 to 99 times 101)
    //   6 digit: 100100, 101101... 999999 (base 100 to 999 times 1001)
    //
    // The pattern is: invalid_id = base × (10^h + 1)
    // where h is half the digit count

    for (start, end) in &cleaned_ranges {
        let max_digits = end.to_string().len();

        // Check each even digit family
        for family in (2..=max_digits).step_by(2) {
            let h = family / 2;

            // Factor converts base to invalid ID
            // h=1 gives 11, so base 5 becomes 55
            // h=2 gives 101, so base 12 becomes 1212
            let factor = 10_i64.pow(h as u32) + 1;

            // Valid bases: 1 to 9 for 2 digit, 10 to 99 for 4 digit, etc
            let min_base = if h == 1 {
                1
            } else {
                10_i64.pow((h - 1) as u32)
            };
            let max_base = 10_i64.pow(h as u32) - 1;

            // Find which bases produce invalid IDs in our range
            // We want: start <= base*factor <= end
            // So: start/factor <= base <= end/factor
            //
            // For range 1000 to 2000 with factor 101:
            // first_base = ceil(1000/101) = 10
            // last_base = floor(2000/101) = 19
            // Gives us 1010, 1111, 1212... 1919
            let first_base = (*start + factor - 1) / factor;
            let last_base = *end / factor;

            // Keep bases within the valid range for this family
            let first_base = first_base.max(min_base);
            let last_base = last_base.min(max_base);

            // Sum using Gauss formula instead of looping
            if first_base <= last_base {
                let count = last_base - first_base + 1;
                let sum_of_bases = count * (first_base + last_base) / 2;
                total += sum_of_bases * factor;
            }
        }
    }

    total
}

fn main() {
    println!(" Day 02 ");
    println!("Part 1: {}", part1(INPUT));
}
