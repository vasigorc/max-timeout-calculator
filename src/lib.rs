use rstest::rstest;
use std::time::Duration;

pub struct MaxTimeoutCalculator {
    min_backoff: Duration,
    random_factor: f64,
    capped_total_wait: Duration,
}

impl MaxTimeoutCalculator {
    fn tail_optimized(
        current_max_backoff: Duration,
        current_sum: Duration,
        factor: f64,
        limit: Duration,
    ) -> Duration {
        if current_sum >= limit {
            current_max_backoff
        } else {
            let new_current_max_backoff = current_max_backoff.mul_f64(factor);
            Self::tail_optimized(
                new_current_max_backoff,
                Duration::from_secs(current_sum.as_secs() + new_current_max_backoff.as_secs()),
                factor,
                limit,
            )
        }
    }

    pub fn calculate_max_backoff(&self) -> Result<Duration, String> {
        let result = Self::tail_optimized(
            self.min_backoff,
            self.min_backoff,
            self.random_factor,
            self.capped_total_wait,
        );
        return Ok(result);
    }

    pub fn new(minb: &Duration, rf: f64, ctw: &Duration) -> MaxTimeoutCalculator {
        if minb.le(&Duration::ZERO) || ctw.le(&Duration::ZERO) {
            panic!("min_backoff and capped_total_wait need both be greater than 0")
        }

        if ctw.lt(minb) {
            panic!("Capped total wait cannot be less than min timeout")
        }

        if rf.le(&1.0) {
            panic!("random_factor must be a floating number greater than 1.0")
        }

        MaxTimeoutCalculator {
            min_backoff: *minb,
            random_factor: rf,
            capped_total_wait: *ctw,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::fixture;

    use super::*;

    #[fixture]
    fn fixture() -> MaxTimeoutCalculator {
        MaxTimeoutCalculator::new(&Duration::new(5, 0), 1.5, &Duration::new(10, 0))
    }

    #[rstest]
    fn it_returns_positive_duration(fixture: MaxTimeoutCalculator) {
        let result = fixture.calculate_max_backoff().unwrap();
        assert!(result.gt(&Duration::ZERO));
    }

    #[rstest]
    fn it_returns_duration_ge_than_min_backoff(fixture: MaxTimeoutCalculator) {
        let result = fixture.calculate_max_backoff().unwrap();
        assert!(result.ge(&fixture.min_backoff))
    }

    #[rstest]
    fn it_returns_duration_le_than_capped_total_wait(fixture: MaxTimeoutCalculator) {
        let result = fixture.calculate_max_backoff().unwrap();
        assert!(result.le(&fixture.capped_total_wait))
    }

    #[rstest]
    #[should_panic]
    fn cannot_build_a_max_timeout_calculator_when_min_backoff_is_le_zero() {
        MaxTimeoutCalculator::new(&Duration::ZERO, 1.0, &Duration::new(5, 0));
    }

    #[rstest]
    #[should_panic]
    fn cannot_build_a_max_timeout_calculator_when_capped_total_wait_le_zero() {
        MaxTimeoutCalculator::new(&Duration::new(5, 0), 1.0, &Duration::ZERO);
    }

    #[rstest]
    #[should_panic]
    fn cannot_build_a_max_timeout_calculator_when_capped_total_wait_is_less_than_min_backoff() {
        MaxTimeoutCalculator::new(&Duration::new(10, 0), 1.0, &Duration::new(5, 0));
    }

    #[rstest]
    #[should_panic]
    fn cannot_build_a_max_timeout_calculator_when_random_factor_is_le_than_one() {
        MaxTimeoutCalculator::new(&Duration::new(5, 0), 1.0, &Duration::new(10, 0));
    }

    #[rstest]
    fn it_should_be_greater_than_random_factor_times_min_backoff(fixture: MaxTimeoutCalculator) {
        let result = fixture.calculate_max_backoff().unwrap();
        assert!(result.ge(&fixture.min_backoff.mul_f64(fixture.random_factor)))
    }
}
