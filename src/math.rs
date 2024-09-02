use std::time::Duration;

pub fn avg(times: &[Duration]) -> Option<Duration> {
    if times.is_empty() {
        return None
    }

    let sum: Duration = times.iter().sum();
    Some(sum / times.len() as u32)
}

pub fn std(times: &[Duration]) -> Option<Duration> {
    let avg = avg(&times)?;

    let variance = times.iter()
        .map(|&time| {
            let diff = if time > avg { time - avg } else { avg - time };
            diff.as_millis() * diff.as_millis()
        })
        .sum::<u128>() / times.len() as u128;

    let std = (variance as f64).sqrt();

    Some(Duration::from_millis(std as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avg_with_empty_vector() {
        let empty_vec = vec![];
        assert_eq!(avg(&empty_vec), None);
    }

    #[test]
    fn avg_with_multiple_elements() {
        let times = vec![
            Duration::from_millis(10000),
            Duration::from_millis(20000),
            Duration::from_millis(30000),
        ];

        assert_eq!(avg(&times), Some(Duration::from_millis(20000)));

        let times = vec![
            Duration::from_millis(2100),
            Duration::from_millis(4300),
            Duration::from_millis(13900),
            Duration::from_millis(5640),
        ];

        assert_eq!(avg(&times), Some(Duration::from_millis(6485)));
    }

    #[test]
    fn std_with_empty_vector() {
        let empty_vec = vec![];
        assert_eq!(std(&empty_vec), None);
    }

    #[test]
    fn std_with_multiple_elements() {
        let times = vec![
            Duration::from_millis(10000),
            Duration::from_millis(20000),
            Duration::from_millis(30000),
        ];

        assert_eq!(std(&times), Some(Duration::from_millis(8164)));

        let times = vec![
            Duration::from_millis(2100),
            Duration::from_millis(4300),
            Duration::from_millis(13900),
            Duration::from_millis(5640),
        ];

        assert_eq!(std(&times), Some(Duration::from_millis(4463)));
    }
}
