pub fn solve(input: &str) {
    let (valid_ranges, product_ids) = parse_input(input);
    
    let part1_result = count_valid_products(&valid_ranges, &product_ids);
    println!("Part 1 - Valid products: {}", part1_result);
    
    let part2_result = count_all_valids(&valid_ranges);
    println!("Part 2 - All valids: {}", part2_result);
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut valid_ranges = Vec::new();
    let mut product_ids = Vec::new();
    let mut parsing_pairs = true;
    
    for line in input.lines() {
        // Empty line marks transition from valid_ranges to product_ids
        if line.is_empty() {
            parsing_pairs = false;
            continue;
        }
        
        if parsing_pairs {
            if let Some((left, right)) = line.split_once('-') {
                let bottom = left.parse::<u64>().unwrap();
                let top = right.parse::<u64>().unwrap();
                valid_ranges.push((bottom, top));
            }
        } else {
            if let Ok(num) = line.parse::<u64>() {
                product_ids.push(num);
            }
        }
    }
    
    valid_ranges.sort_by_key(|&(bottom, _)| bottom);
    
    product_ids.sort_by_key(|&x| std::cmp::Reverse(x));
    
    (valid_ranges, product_ids)
}

fn count_valid_products(valid_ranges: &[(u64, u64)], product_ids: &[u64]) -> u64 {
    let mut num_valids = 0;
    let min_range_bottom = valid_ranges[0].0;
    
    'outer: for &product_id in product_ids {
        if product_id < min_range_bottom {
            break 'outer;
        }
        
        for &(bottom, top) in valid_ranges {
            if product_id >= bottom && product_id <= top {
                num_valids += 1;
                continue 'outer;
            }
        }
    }
    
    num_valids
}

fn count_all_valids(valid_ranges: &[(u64, u64)]) -> u64 {
    if valid_ranges.is_empty() {
        return 0;
    }
    
    let mut merged_ranges = Vec::new();
    let mut global_bottom = valid_ranges[0].0;
    let mut global_top = valid_ranges[0].1;
    
    // Loop starting from second range
    for &(bottom, top) in &valid_ranges[1..] {
        if global_top < bottom {
            // Split: new range detected
            merged_ranges.push((global_bottom, global_top));
            global_bottom = bottom;
            global_top = top;
            continue;
        }
        if global_top >= top {
            // Current range is entirely within global, ignore it
            continue;
        } else {
            // Ranges overlap, extend global_top
            global_top = top;
        }
    }
    
    // Append the last range
    merged_ranges.push((global_bottom, global_top));
    
    // Count all valid IDs in merged ranges
    merged_ranges.iter().map(|(bottom, top)| top - bottom + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "47-53\n37-100\n1-10\n\n300\n100\n200";
        let (valid_ranges, product_ids) = parse_input(input);
        
        assert_eq!(valid_ranges.len(), 3);
        assert_eq!(valid_ranges[0], (1, 10));
        assert_eq!(valid_ranges[1], (37, 100));
        assert_eq!(valid_ranges[2], (47, 53));
        
        assert_eq!(product_ids.len(), 3);
        assert_eq!(product_ids[0], 300);
        assert_eq!(product_ids[1], 200);
        assert_eq!(product_ids[2], 100);
    }
    
    #[test]
    fn test_count_valid_products() {
        let valid_ranges = vec![(10, 100)];
        let product_ids = vec![101, 100, 10, 9, 5, 1];
        
        let result = count_valid_products(&valid_ranges, &product_ids);
        assert_eq!(result, 2);
    }
    
    #[test]
    fn test_count_valid_products_multiple_ranges() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (valid_ranges, product_ids) = parse_input(input);
        
        let result = count_valid_products(&valid_ranges, &product_ids);
        assert_eq!(result, 3);
    }
    
    #[test]
    fn test_count_all_valids() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (valid_ranges, _) = parse_input(input);
        
        let result = count_all_valids(&valid_ranges);
        assert_eq!(result, 14);
    }
}
