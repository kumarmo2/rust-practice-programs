  server  = true,
  datacenter = "mydc",
  node_name = "node2",
  bind_addr = "0.0.0.0",
  data_dir = "/tmp/consul",
  bootstrap_expect = 3,
  retry_join=["192.168.122.80", "192.168.122.247", "192.168.122.168"],
  addresses =  {
    http = "0.0.0.0",
    grpc = "0.0.0.0",
  },
  log_level = "INFO",
  auto_reload_config = true,
  ui_config = {
    enabled = true
  }
