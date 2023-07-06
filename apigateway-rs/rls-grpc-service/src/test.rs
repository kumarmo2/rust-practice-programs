use crate::{throttler::Throttler, HttpMethod, RateLimitConfig, TimeUnit};

// NOTE: you
#[test]
fn empty_configs_return_empty_vec() {
    let throttler = Throttler::new(vec![]);
    assert_eq!(
        0,
        throttler
            .get_eligible_rate_limit_configs("/api/", HttpMethod::Get)
            .collect::<Vec<_>>()
            .len()
    );
}

#[test]
fn root_path_prefix_tests() {
    let throttler = Throttler::new(vec![RateLimitConfig {
        api_path_prefix: "/".to_string(),
        method: HttpMethod::Get,
        window: 1,
        max_requests: 5,
        time_unit: TimeUnit::M,
    }]);

    assert_eq!(
        1,
        throttler
            .get_eligible_rate_limit_configs("/api/", HttpMethod::Get)
            .collect::<Vec<_>>()
            .len()
    );

    assert_eq!(
        1,
        throttler
            .get_eligible_rate_limit_configs("/", HttpMethod::Get)
            .collect::<Vec<_>>()
            .len()
    );

    let throttler = Throttler::new(vec![RateLimitConfig {
        api_path_prefix: "/".to_string(),
        method: HttpMethod::Post,
        window: 1,
        max_requests: 5,
        time_unit: TimeUnit::M,
    }]);

    // since config is for post, and request method is get
    assert_eq!(
        0,
        throttler
            .get_eligible_rate_limit_configs("/api/", HttpMethod::Get)
            .collect::<Vec<_>>()
            .len()
    );
}
