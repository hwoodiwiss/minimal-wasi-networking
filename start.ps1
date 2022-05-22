[CmdletBinding()]
param (
	[Parameter()]
	[String]
	$Config = "debug",
	[Parameter()]
	[String]
	$Port = "25565"
	[Parameter()]
	[String]
	$Port2 = "25566"
)

wasmtime --tcplisten=127.0.0.1:$Port --tcplisten=127.0.0.1:$Port2 .\target\wasm32-wasi\$Config\wasi-net-test.wasm