#![allow(dead_code)]
use crate::{HttpMethod, RateLimitConfig};

pub(crate) struct Throttler {
    rate_limit_configs: Vec<RateLimitConfig>,
}

impl Throttler {
    pub(crate) fn new(mut rate_limit_configs: Vec<RateLimitConfig>) -> Self {
        /*Sanitizing: just in case if any prefix ends with '/', remove that.
         * Also, while matching with request, will do with request_path.*/
        for config in rate_limit_configs.iter_mut() {
            if config.api_path_prefix.ends_with("/") {
                config.api_path_prefix.pop();
            }
        }
        Self { rate_limit_configs }
    }
}

impl Throttler {
    pub(crate) fn get_eligible_rate_limit_configs<'a>(
        &'a self,
        mut request_api_path: &'a str,
        request_method: HttpMethod,
    ) -> impl Iterator<Item = &'a RateLimitConfig> {
        self.rate_limit_configs.iter().filter(move |config| {
            if config.method != request_method {
                return false;
            }
            if request_api_path.ends_with("/") {
                request_api_path = &request_api_path[0..(request_api_path.len() - 1)];
            }

            if !request_api_path.starts_with(&config.api_path_prefix) {
                return false;
            }
            true
        })
    }
}

// NOTE: you can assume the request_paths and prefix will be correct http paths/prefixes.
// pub(crate) fn get_eligible_rate_limit_configs<'config>(
// rate_limit_configs: &'config Vec<RateLimitConfig>,
// request_api_path: &'config str,
// method: HttpMethod,
// ) -> Vec<&'config RateLimitConfig> {
// // rate_limit_configs.iter().collect()

// todo!()
// }
