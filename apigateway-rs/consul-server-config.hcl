  server  = true,
  datacenter = "mydc",
  node_name = "node2",
  bind_addr = "0.0.0.0",
  data_dir = "/tmp/consul",
  bootstrap_expect = 3,
  addresses =  {
    http = "0.0.0.0",
    grpc = "0.0.0.0",
  },
  log_level = "INFO",
  auto_reload_config = true,
  ui_config = {
    enabled = true
  }
