if (Test-Path -Path './docs') {
    Remove-Item -Recurse -Force './docs'
}

reveal-md rust_code_snippets.md --static docs
Copy-Item -Recurse -Force css docs