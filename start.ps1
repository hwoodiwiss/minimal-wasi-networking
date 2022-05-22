[CmdletBinding()]
param (
	[Parameter()]
	[String]
	$Config = "debug"
)

wasmtime --tcplisten=127.0.0.1:25565 .\target\wasm32-wasi\$Config\wasi-net-test.wasm