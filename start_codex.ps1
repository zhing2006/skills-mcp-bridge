# Ensure CODEX_HOME points at the .codex directory under the current working directory.
$env:CODEX_HOME = Join-Path (Get-Location) ".codex"

$mode = ""
$modeLabel = ""
$arguments = $args

if ($arguments.Count -gt 0) {
    switch ($arguments[0]) {
        "-a1" {
            $mode = "--full-auto"
            $modeLabel = "full-auto"
            $arguments = @($arguments | Select-Object -Skip 1)
        }
        "-a0" {
            $mode = "--dangerously-bypass-approvals-and-sandbox"
            $modeLabel = "dangerously-bypass-approvals-and-sandbox"
            $arguments = @($arguments | Select-Object -Skip 1)
        }
    }
}

$cmd = @("codex")
if ($mode -ne "") {
    Write-Host "Launching Codex in " -NoNewline
    Write-Host $modeLabel -ForegroundColor Green -NoNewline
    Write-Host " mode..."
    $cmd += $mode
}
if ($arguments.Count -gt 0) {
    $cmd += $arguments
}

$cmdArgs = $cmd[1..($cmd.Count - 1)]
& $cmd[0] @cmdArgs
