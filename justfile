set windows-shell := ["powershell.exe", "-C"]

oc := 'odin'
exe := 'bin/pingky.exe'


cmd := oc + ' build app -out:' + exe
    
debug:
    {{cmd}} -debug

release:
    {{cmd}} -o:speed -subsystem:windows

run:
    {{exe}}
