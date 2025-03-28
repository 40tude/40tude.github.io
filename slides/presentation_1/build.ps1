if (Test-Path -Path './docs') {
    Remove-Item -Recurse -Force './docs'
}

reveal-md slides_01.md --static docs
Copy-Item -Recurse -Force css docs