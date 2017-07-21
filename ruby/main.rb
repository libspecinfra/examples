require 'libspecinfra'
require 'libspecinfra/backend/direct'

b = Libspecinfra::Backend::Direct::Binding.new()
s = Libspecinfra::Specinfra::Binding.new(b)
f = s.file("/etc/passwd")

printf("%#o", f.mode)
