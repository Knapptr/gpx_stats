pub fn get_elevation_change(eles: &Vec<f32>) -> (f32, f32) {
    let mut gain = 0f32;
    let mut loss = 0f32;
    let mut prev: Option<f32> = None;
    for ele in eles {
        if let Some(previous_elevation) = prev {
            let diff = (previous_elevation - ele).abs();
            if previous_elevation < *ele {
                gain += diff;
            }
            if previous_elevation > *ele {
                loss += diff;
            }
        }
        prev = Some(*ele)
    }
    (gain, loss)
}

pub fn smooth_ele_data(eles: Vec<f32>, mut sample_size: usize) -> Vec<f32> {
    // establish maximum index
    let max_index = eles.len() - 1;
    // handle odd sample_size
    sample_size = if sample_size % 2 == 0 {
        sample_size + 1
    } else {
        sample_size
    };
    // number of samples on either side of center point
    let each_side = (sample_size - 1) / 2;
    // init bound indexes
    let mut lower_bound = 0usize;
    let mut higher_bound = if lower_bound + each_side > (max_index) {
        max_index
    } else {
        lower_bound + each_side
    };
    // init running sum to avg of first point in sample bounds
    let mut sum: f32 = eles.iter().take(each_side + 1).sum::<f32>()
        + (eles.iter().take(1).sum::<f32>() * each_side as f32);

    // init return vec
    let mut smoothed_data = vec![sum / sample_size as f32];
    // sum -= eles.iter().take(1).sum::<f32>();

    // println!("Each Side: {} | Sample Size: {}\n", each_side, sample_size);
    ////
    ////
    for (current_index, _element) in eles.iter().enumerate().skip(1) {
        // println!(
        // "\nAt: {} ({})\nSubtracting {} from sum of {}",
        // _element, current_index, eles[lower_bound], sum
        // );
        sum -= eles[lower_bound];
        // move lower_bound up 1 if significant
        lower_bound = if current_index as isize - each_side as isize >= 0 {
            current_index - each_side
        } else {
            0
        };
        // move higher bound if possible
        higher_bound = max_index.min(higher_bound + 1);
        sum += eles[higher_bound];
        // println!(
        //     "Sum: {} | lower bound: {} | upper bound: {}",
        //     sum, lower_bound, higher_bound
        // );
        smoothed_data.push(sum / sample_size as f32);
    }
    ////
    ////
    smoothed_data
}

#[cfg(test)]
#[test]
fn smoothed_data_transform() {
    let test_data: Vec<f32> = vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0];
    let smoothed_data = smooth_ele_data(test_data, 2);
    let expected_data = vec![2.67, 4.0, 6.0, 8.0, 10.0, 11.33];

    // Comparing rounded strings here, because I can't figure out a better way to compare
    // floating points rn.
    for (i, el) in smoothed_data.iter().enumerate() {
        let rounded_smoothed = format!("{:.2}", el);
        // println!("comparing {} to {}", expected_data[i], rounded_smoothed);
        assert_eq!(rounded_smoothed, format!("{:.2}", expected_data[i]));
    }
}
