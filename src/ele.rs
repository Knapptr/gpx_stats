pub fn get_elevation_change<T: Iterator<Item = f32>>(eles: T) -> (f32, f32) {
    let mut gain = 0f32;
    let mut loss = 0f32;
    let mut prev: Option<f32> = None;
    for ele in eles {
        if let Some(previous_elevation) = prev {
            let diff = (previous_elevation - ele).abs();
            if diff > 10f32 {
                if previous_elevation < ele {
                    gain += ele;
                }
                if previous_elevation > ele {
                    loss += ele;
                }
            }
        }
        prev = Some(ele)
    }
    (gain, loss)
}
