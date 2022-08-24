pub fn get_closest_checkpoint(chain_name: &str, height: u64) -> Option<(u64, &'static str, &'static str)> {
    log::info!("Trying to get checkpoint closest to block {}", height);
    match chain_name {
        "ztestsapling" => get_test_checkpoint(height),
        "zs" | "main" => get_main_checkpoint(height),
        _ => None,
    }
}

fn get_test_checkpoint(height: u64) -> Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![

    ];

    find_checkpoint(height, checkpoints)
}

pub fn get_all_main_checkpoints() -> Vec<(u64, &'static str, &'static str)> {
    vec![
        (200000, "0000000097934aab530005e99148224c3c863f2c785e4934645972dd7c611f70",
                 "01395c63eaba3c8557d9dd0e065d14ded04cfd716483db4fe8562ad9e28527114301af3a227432535ecdfca769b6cd15995aebe73e63e8e60b160ad0bdb6a2000c6e1100016cea12e77e0935812534a7142e83be32928c29064e1d79ba385546ab7470b0500000010555af98501d9db6922f3d58bddb1b4a9388be80edfc0c24fd27b8704c5a3a3d0194c07ff8eb9c31492e765636e013127be3e7d4f57109523b2836525e1a67910800013ce632ebfc9371015d9b417ad17273904b5f3d60d65f346ab33b814b276c0b54013661f29c788af7eac0a51a1e0a26be268b5f27cf7b90a4038d60a894aff07945000001830a9e6160576d1d88b4821c63221e0d3cce2466b2d3e168a33ee9f10113a33a014b1a2284e2536b72e62b802d610494a0bce24f2d4529ecaf62eb5b4bbd9ff5240100400289d1673c3f65f945a1554df84890e42cbb859e46b4b2c4fd61b8c2c63c0000014a069a106e49d04eaf2aaf969d6af60421bdc5d0ae34441cf0ff78fda6ec3670"
        ),

    ]
}

fn get_main_checkpoint(height: u64) -> Option<(u64, &'static str, &'static str)> {
    find_checkpoint(height, get_all_main_checkpoints())
}

fn find_checkpoint(
    height: u64,
    chkpts: Vec<(u64, &'static str, &'static str)>,
) -> Option<(u64, &'static str, &'static str)> {
    // Find the closest checkpoint
    let mut heights = chkpts.iter().map(|(h, _, _)| *h as u64).collect::<Vec<_>>();
    heights.sort();

    match get_first_lower_than(height, heights) {
        Some(closest_height) => chkpts.iter().find(|(h, _, _)| *h == closest_height).map(|t| *t),
        None => None,
    }
}

fn get_first_lower_than(height: u64, heights: Vec<u64>) -> Option<u64> {
    // If it's before the first checkpoint, return None.
    if heights.len() == 0 || height < heights[0] {
        return None;
    }

    for (i, h) in heights.iter().enumerate() {
        if height < *h {
            return Some(heights[i - 1]);
        }
    }

    return Some(*heights.last().unwrap());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_lower_than() {
        assert_eq!(get_first_lower_than(9, vec![10, 30, 40]), None);
        assert_eq!(get_first_lower_than(10, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(11, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(29, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(30, vec![10, 30, 40]).unwrap(), 30);
        assert_eq!(get_first_lower_than(40, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(41, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(99, vec![10, 30, 40]).unwrap(), 40);
    }

    #[test]
    fn test_checkpoints() {
        assert_eq!(get_test_checkpoint(500000), None);
        assert_eq!(get_test_checkpoint(600000).unwrap().0, 600000);
        assert_eq!(get_test_checkpoint(625000).unwrap().0, 600000);
        assert_eq!(get_test_checkpoint(650000).unwrap().0, 650000);
        assert_eq!(get_test_checkpoint(655000).unwrap().0, 650000);

        assert_eq!(get_main_checkpoint(500000), None);
        assert_eq!(get_main_checkpoint(610000).unwrap().0, 610000);
        assert_eq!(get_main_checkpoint(625000).unwrap().0, 610000);
    }
}
