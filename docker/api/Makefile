# Makefile for the API into Docker.
# TODO:
# Change Go build target
# -----------------------------------
# container.go
container_api: container_api.a  
        go tool link -o container_api -extld clang -buildmode exe -buildid b01dca11ab1e -linkmode external -v container_api.a

container_api.a: container.go container.o  
        go tool compile -o container_api.a -p main -buildid b01dca11ab1e -pack container.go
        go tool pack r rustgo.a hello.o

container.o: container.s  
        go tool asm -I "$(shell go env GOROOT)/pkg/include" -D GOOS_darwin -D GOARCH_amd64 -o hello.o hello.s
        
rusty_container.a: container.rs  
        rustc -g -O --crate-type staticlib hello.rs
		
container_api: container_api.a rusty_container.a  
        go tool link -o rustgo -extld clang -buildmode exe -buildid b01dca11ab1e -linkmode external -v -extldflags='-lhello -L"$(CURDIR)"' rustgo.a
