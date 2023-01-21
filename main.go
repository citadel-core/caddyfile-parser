package main

import (
	"fmt"
	"os"

	"github.com/caddyserver/caddy/v2/caddyconfig/caddyfile"
	"github.com/caddyserver/caddy/v2/caddyconfig/httpcaddyfile"

	_ "github.com/caddyserver/caddy/v2/modules/standard"
	_ "github.com/runcitadel/caddy-runningcitadel"
)

import "C"

//export ParseCaddyfile
func ParseCaddyfile(filename string, contents []byte, length *uint16) *C.char {
	var adapter = caddyfile.Adapter{ServerType: httpcaddyfile.ServerType{}}
	res, warnings, error := adapter.Adapt(contents, map[string]interface{}{"filename": filename})
	if warnings == nil {
		fmt.Println("No warnings")
	} else if len(warnings) > 0 {
		// Print warnings
		for _, warning := range warnings {
			fmt.Println(warning)
		}
	}

	if error != nil {
		*length = uint16(len(error.Error()))
		return C.CString(error.Error())
	}

	*length = uint16(len(res))
	return C.CString(string(res))
}

//export FormatCaddyfile
func FormatCaddyfile(contents []byte, length *uint16) *C.char {
	bytes := caddyfile.Format(contents)
	*length = uint16(len(bytes))
	return C.CString(string(bytes))
}

func main() {
	fmt.Println("Hello, World!")
	args := os.Args[1:]
	if len(args) != 1 {
		fmt.Println("Usage: caddyfile-to-json <filename>")
		return
	}
	filename := args[0]

	// Step 2: Read the file
	contents, error := os.ReadFile(filename)
	if error != nil {
		fmt.Println(error)
		return
	}

	// Step 3: Call ParseCaddyfile
	var length uint16
	result := ParseCaddyfile(filename, contents, &length)

	fmt.Println(C.GoString((*C.char)(result)))
}
