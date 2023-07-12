#![allow(dead_code)]
use redis::{Client, RedisResult};

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

    pub(crate) async fn should_throttle<'a>(
        &'a self,
        request_api_path: &'a str,
        request_method: HttpMethod,
        client_id: &str,
        redis: &Client,
    ) -> RedisResult<bool> {
        let mut eligible_configs = self
            .get_eligible_rate_limit_configs(request_api_path, request_method)
            .collect::<Vec<_>>();

        if eligible_configs.len() < 1 {
            return Ok(false);
        }
        let Ok(mut conn) = redis.get_async_connection().await else {
            println!("could not make connection to redis");
            return Ok(false);
        };
        let lua_script = Throttler::get_script();

        eligible_configs.sort_by(|first, second| {
            first
                .get_window_in_seconds()
                .cmp(&second.get_window_in_seconds())
        });

        let num_of_windows = eligible_configs.len();
        let mut args = vec![num_of_windows as u32];
        let mut keys = vec![];
        println!("num_of_windows: {}", num_of_windows);
        for config in eligible_configs.iter() {
            args.push(config.get_window_in_seconds());
            args.push(config.max_requests);
            keys.push(config.get_config_key_for_client(client_id));
        }

        for arg in args.iter() {
            println!("arg: {}", arg);
        }

        for key in keys.iter() {
            println!("key: {}", key);
        }

        // lua_script.invoke_async(&mut conn).await
        lua_script.arg(args).key(keys).invoke_async(&mut conn).await
    }

    fn get_script() -> redis::Script {
        redis::Script::new(
            r#"
            local current_time = redis.call('TIME')
            local num_windows = ARGV[1]
            for i=2, num_windows*2, 2 do
                local window = ARGV[i]
                local max_requests = ARGV[i+1]
                local curr_key = KEYS[i/2]
                local trim_time = tonumber(current_time[1]) - window
                redis.call('ZREMRANGEBYSCORE', curr_key, 0, trim_time)
                local request_count = redis.call('ZCARD',curr_key)
                if request_count >= tonumber(max_requests) then
                    return 1
                end
            end
            for i=2, num_windows*2, 2 do
                local curr_key = KEYS[i/2]
                local window = ARGV[i]
                redis.call('ZADD', curr_key, current_time[1], current_time[1] .. current_time[2])
                redis.call('EXPIRE', curr_key, window)
            end
            return 0
           "#,
        )
    }
}
