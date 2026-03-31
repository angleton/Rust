# Rust gRPC API with Go stub generation

This project provides a minimal Rust gRPC server using `tonic` and a `.proto` contract that can also generate Go RPC stubs.

## Included RPC

- `Greeter.SayHello(HelloRequest) -> HelloReply`

## Run the Rust server

```powershell
cargo run
```

The server listens on `127.0.0.1:50051`.

## Generate Go stubs

Install these first:

```powershell
# Install Go from https://go.dev/dl/
# Install protoc from https://grpc.io/docs/protoc-installation/
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
```

Make sure `%USERPROFILE%\go\bin` is on your `PATH`, then run:

```powershell
.\scripts\generate-go.ps1
```

Generated files will be written to `gen/go`.

## Contract location

- Protobuf definition: `proto/hello.proto`
- Rust server entrypoint: `src/main.rs`
