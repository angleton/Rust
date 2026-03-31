param(
    [string]$ProtoDir = "proto",
    [string]$OutDir = "gen/go"
)

$ErrorActionPreference = "Stop"

if (-not (Get-Command protoc -ErrorAction SilentlyContinue)) {
    throw "protoc was not found. Install Protocol Buffers first."
}

if (-not (Get-Command protoc-gen-go -ErrorAction SilentlyContinue)) {
    throw "protoc-gen-go was not found. Run: go install google.golang.org/protobuf/cmd/protoc-gen-go@latest"
}

if (-not (Get-Command protoc-gen-go-grpc -ErrorAction SilentlyContinue)) {
    throw "protoc-gen-go-grpc was not found. Run: go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest"
}

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

protoc `
  --proto_path=$ProtoDir `
  --go_out=$OutDir `
  --go_opt=paths=source_relative `
  --go-grpc_out=$OutDir `
  --go-grpc_opt=paths=source_relative `
  "$ProtoDir/hello.proto"

Write-Host "Go stubs generated under $OutDir"
