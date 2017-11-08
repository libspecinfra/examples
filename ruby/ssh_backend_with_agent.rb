require 'libspecinfra'
require 'libspecinfra/backend/ssh'

b = Libspecinfra::Backend::SSH::Binding.new("localhost", "mizzy")
s = Libspecinfra::Specinfra::Binding.new(b)
f = s.file("/etc/passwd")

printf("%#o", f.mode)
