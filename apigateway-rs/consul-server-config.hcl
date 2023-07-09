  server  = true,
  datacenter = "mydc",
  node_name = "node2",
  bind_addr = "0.0.0.0",
  data_dir = "/tmp/consul",
<<<<<<< HEAD
  bootstrap_expect = 2,
  retry_join=["192.168.122.80", "192.168.122.247", "192.168.122.168"],
=======
  bootstrap_expect = 3,
>>>>>>> 97c92f71ff3460f8dd2c0fe391ffc95431405ff8
  addresses =  {
    http = "0.0.0.0",
    grpc = "0.0.0.0",
  },
  log_level = "INFO",
  auto_reload_config = true,
  ui_config = {
    enabled = true
  }
