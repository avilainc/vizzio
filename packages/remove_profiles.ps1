$baseDirs = @('avila', 'avl', 'avx')
$updatedFiles = @()

foreach ($baseDir in $baseDirs) {
    if (-not (Test-Path $baseDir)) { continue }

    $cargoFiles = Get-ChildItem -Path $baseDir -Recurse -Filter "Cargo.toml"

    foreach ($file in $cargoFiles) {
        $content = Get-Content $file.FullName -Raw
        $originalContent = $content

        # Remove secoes [profile.*] e seu conteudo ate a proxima secao
        $content = $content -replace '(?ms)\n\[profile\.[^\]]+\][^\[]*(?=\[|\z)', ''

        # Remove linhas vazias extras
        $content = $content -replace '(?ms)\n\n\n+', "`n`n"

        if ($content -ne $originalContent) {
            Set-Content -Path $file.FullName -Value $content.TrimEnd() -NoNewline
            Add-Content -Path $file.FullName -Value "`n"
            $updatedFiles += $file.FullName
            Write-Host "Updated: $($file.FullName)"
        }
    }
}

Write-Host ""
Write-Host "Total files updated: $($updatedFiles.Count)"
