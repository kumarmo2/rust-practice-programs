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

#[test]
fn multiple_rate_limit_configs_test() {
    let throttler = Throttler::new(vec![
        RateLimitConfig {
            api_path_prefix: "/api/v1/foo".to_string(),
            method: HttpMethod::Get,
            window: 1,
            max_requests: 5,
            time_unit: TimeUnit::M,
        },
        RateLimitConfig {
            api_path_prefix: "/api/v1/foo".to_string(),
            method: HttpMethod::Post,
            window: 1,
            max_requests: 5,
            time_unit: TimeUnit::M,
        },
        RateLimitConfig {
            api_path_prefix: "/api/v2".to_string(),
            method: HttpMethod::Get,
            window: 1,
            max_requests: 5,
            time_unit: TimeUnit::M,
        },
        RateLimitConfig {
            api_path_prefix: "/api/v1/".to_string(),
            method: HttpMethod::Get,
            window: 1,
            max_requests: 5,
            time_unit: TimeUnit::M,
        },
    ]);

    let eligible_configs = throttler
        .get_eligible_rate_limit_configs("/api/v1/foo/1/", HttpMethod::Get)
        .collect::<Vec<_>>();

    assert_eq!(2, eligible_configs.len());

    let config = &eligible_configs[0];
    assert_eq!(config.method, HttpMethod::Get);
    assert_eq!(config.api_path_prefix, "/api/v1/foo");

    let config = &eligible_configs[1];
    assert_eq!(config.method, HttpMethod::Get);
    assert_eq!(config.api_path_prefix, "/api/v1"); // Note that i have removed trailing '/' as it
                                                   // is removed as part of sanitization during throttler initialization.
}
