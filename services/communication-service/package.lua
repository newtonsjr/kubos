return {
  name = "kubos/kubos-communication-service",
  version = "0.0.4",
  description = "Service to route udp packets to and from a custom transport.",
  tags = { "kubos", "udp", "nat", "stdio", "serial" },
  author = { name = "Tim Caswell", email = "tim@kubos.co" },
  homepage = "https://github.com/kubos/kubos",
  luvi = {
    flavor = "tiny",
    inline = "#!/home/system/usr/bin/luvi-regular --\n"
  },
  dependencies = {
    "luvit/require",
    "luvit/pretty-print",
    "luvit/json",
    "luvit/secure-socket",
    "creationix/base64",
    "creationix/coro-wrapper",
    "creationix/coro-channel",
    "creationix/coro-fs",
    "creationix/coro-websocket",
    "creationix/toml",
  },
  files = {
    "**.lua",
  },
  license = "Apache 2.0"
}