const MICROSECOND: f64 = 1000000.0;
const MILLISECOND: f64 = 1000.0;

#[derive(Debug, Clone)]
pub struct MathUtil {
    // count : u64,
    // sum_interval: f64,
    max: f64,
    min: f64,
    latest_time: i64,
}

impl MathUtil {
    pub fn new() -> Self {
        MathUtil {
            // count: 0,
            // sum_interval: 0.0,
            max: 0.0f64,
            min: f64::MAX,
            latest_time: 0,
        }
    }

    pub fn calc_frequency<T: PartialEq>(&mut self, logs: Vec<(i64, T)>) -> Option<(f64, f64, f64, f64)> {
        let mut sum: f64 = 0.0;
        let mut valid_count: u64 = 0;
        logs.windows(2)
            .filter(|pair| {pair[1].0 > self.latest_time })
            .for_each(|pair| {
                let log = &pair[1];
                let prev_log = &pair[0];
                // 若前后数据点相同, 判定为重复并移除
                if &log.1 == &prev_log.1 {
                    return;
                }
                let delta = (log.0 - prev_log.0) as f64;
                sum += delta as f64;
                valid_count += 1;
                if delta < self.min {
                    self.min = delta;
                }
                if delta > self.max {
                    self.max = delta;
                }
            });
        // self.latest_time = logs.last().unwrap().0;
        let avg_interval: f64 = sum / valid_count as f64;
        // self.count += valid_count;
        // self.sum_interval += sum;
        Some((
            MICROSECOND / avg_interval,
            MICROSECOND / self.max,
            MICROSECOND / self.min,
            avg_interval / MILLISECOND,
        ))
    }
}