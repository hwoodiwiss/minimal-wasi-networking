[CmdletBinding()]
param (
	[Parameter()]
	[String]
	$Config = "debug",
	[Parameter()]
	[String]
	$Port = "25565"
)

wasmtime --tcplisten=127.0.0.1:$Port .\target\wasm32-wasi\$Config\wasi-net-test.wasm