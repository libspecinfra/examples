b = Libspecinfra::Backend::Direct.new()
s = Libspecinfra::Specinfra.new(b)
f = s.file("/etc/passwd")

printf("%#o", f.mode)
