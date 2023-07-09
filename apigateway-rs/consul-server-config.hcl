  server  = true,
  datacenter = "mydc",
  node_name = "node1",
  bind_addr = "{{ GetInterfaceIP \"wlp6s0\" }}",
  #bind_addr = "0.0.0.0",
  data_dir = "/tmp/consul",
  bootstrap_expect = 1,
  log_level = "INFO",
  auto_reload_config = true,
  ui_config = {
    enabled = true
  }
