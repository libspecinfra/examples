b = Libspecinfra::Backend::SSH.new("localhost")
s = Libspecinfra::Specinfra.new(b)
f = s.file("/etc/passwd")

printf("%#o", f.mode)
