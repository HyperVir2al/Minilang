# Как запустить этот проект.

Для запуска тестов нужно перейти в папку проекта и запустить скрипт формата .ml

```bash
cd cd "$HOME\OneDrive\Desktop\minilang"
```

для запуска одного файла
```bash
cargo run -- examples/basic.ml
```

для запуска всех
```bash
cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Get-ChildItem examples -Filter *.ml | ForEach-Object {
    Write-Output "== $($_.Name) =="
    .\target\release\minilang.exe $_.FullName
}
```