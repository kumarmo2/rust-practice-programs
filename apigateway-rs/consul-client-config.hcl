  datacenter = "mydc",
  node_name = "node1",
  #bind_addr = "{{ GetInterfaceIP \"enp7s0\" }}",
  bind_addr = "127.0.0.1",
  ports = {
  http = 8601,
  dns = 8700
  }
  data_dir = "/tmp/consul",
  log_level = "INFO",
  auto_reload_config = true,
