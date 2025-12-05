#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Real-time monitoring dashboard for Vizzio platform deployment

.DESCRIPTION
    Monitors all running services, health metrics, and provides live status updates

.EXAMPLE
    .\Monitor-Deployment.ps1
#>

param(
    [int]$RefreshInterval = 5,
    [switch]$Continuous = $true
)

$ErrorActionPreference = "SilentlyContinue"

function Get-ServiceStatus {
    $services = @{}
    $output = docker-compose ps --format "table {{.Service}}\t{{.Status}}\t{{.Ports}}" 2>&1

    $output | ForEach-Object {
        if ($_ -match "^\w+") {
            $parts = $_ -split "\s{2,}"
            if ($parts.Count -ge 2) {
                $service = $parts[0].Trim()
                $status = $parts[1].Trim()
                $ports = if ($parts.Count -gt 2) { $parts[2].Trim() } else { "N/A" }

                $services[$service] = @{
                    Status = $status
                    Ports = $ports
                }
            }
        }
    }

    return $services
}

function Test-ServiceHealth {
    param(
        [string]$Service,
        [int]$Port
    )

    $healthy = $false
    $response = $null

    try {
        switch ($Service) {
            "mongodb" {
                $response = docker-compose exec -T vizzio-mongodb mongosh --eval "db.adminCommand('ping')" 2>&1
                $healthy = $response -match "ok"
            }
            "redis" {
                $response = docker-compose exec -T vizzio-redis redis-cli PING 2>&1
                $healthy = $response -match "PONG"
            }
            "rabbitmq" {
                $response = (Invoke-WebRequest -Uri "http://localhost:15672/api/aliveness-test" -Credential (New-Object PSCredential("admin", (ConvertTo-SecureString "password123" -AsPlainText -Force))) -Method GET 2>&1).StatusCode
                $healthy = $response -eq 200
            }
        }
    }
    catch {
        $healthy = $false
    }

    return $healthy
}

function Format-Dashboard {
    param(
        [hashtable]$Services,
        [int]$IterationCount
    )

    Clear-Host

    Write-Host "╔════════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║                                                                            ║" -ForegroundColor Cyan
    Write-Host "║           VIZZIO PLATFORM - DEPLOYMENT MONITORING DASHBOARD                ║" -ForegroundColor Cyan
    Write-Host "║                                                                            ║" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan

    Write-Host "`n[$(Get-Date -Format 'HH:mm:ss')] Monitoring Cycle: $IterationCount`n" -ForegroundColor Yellow

    Write-Host "SERVICE STATUS:" -ForegroundColor Yellow
    Write-Host "─────────────────────────────────────────────────────────────────────────────" -ForegroundColor Gray

    $serviceMap = @{
        "vizzio-mongodb" = @{ Name = "MongoDB"; Port = 27017 }
        "vizzio-redis" = @{ Name = "Redis"; Port = 6379 }
        "vizzio-rabbitmq" = @{ Name = "RabbitMQ"; Port = 5672 }
    }

    foreach ($svc in $serviceMap.GetEnumerator()) {
        $key = $svc.Key
        $name = $svc.Value.Name
        $port = $svc.Value.Port

        if ($Services.ContainsKey($key)) {
            $status = $Services[$key].Status
            $ports = $Services[$key].Ports

            # Determine health status
            $healthy = Test-ServiceHealth -Service $name.ToLower() -Port $port
            $healthIcon = if ($healthy) { "[✓]" } else { "[!]" }
            $healthColor = if ($healthy) { "Green" } else { "Yellow" }

            $statusDisplay = if ($status -match "Up") { "Running" } else { "Stopped" }
            $statusColor = if ($status -match "Up") { "Green" } else { "Red" }

            Write-Host "  $healthIcon $name" -ForegroundColor $healthColor -NoNewline
            Write-Host " - $statusDisplay" -ForegroundColor $statusColor -NoNewline
            Write-Host " (Port: $port)" -ForegroundColor Gray
            Write-Host "      Ports: $ports" -ForegroundColor Gray
        }
    }

    Write-Host "`nCONNECTION STRINGS:" -ForegroundColor Yellow
    Write-Host "─────────────────────────────────────────────────────────────────────────────" -ForegroundColor Gray
    Write-Host "  MongoDB:  mongodb://admin:password123@localhost:27017/vizzio" -ForegroundColor Cyan
    Write-Host "  Redis:    redis://localhost:6379" -ForegroundColor Cyan
    Write-Host "  RabbitMQ: amqp://admin:password123@localhost:5672" -ForegroundColor Cyan

    Write-Host "`nWEB INTERFACES:" -ForegroundColor Yellow
    Write-Host "─────────────────────────────────────────────────────────────────────────────" -ForegroundColor Gray
    Write-Host "  RabbitMQ Management: http://localhost:15672" -ForegroundColor Cyan
    Write-Host "  Backend API:         http://localhost:3000" -ForegroundColor Cyan
    Write-Host "  Frontend UI:         http://localhost:3001" -ForegroundColor Cyan

    Write-Host "`nDOCKER STATISTICS:" -ForegroundColor Yellow
    Write-Host "─────────────────────────────────────────────────────────────────────────────" -ForegroundColor Gray

    try {
        $stats = docker stats --no-stream --format "table {{.Container}}\t{{.MemUsage}}\t{{.CPUPerc}}" 2>&1 | Select-String "vizzio"
        if ($stats) {
            $stats | ForEach-Object {
                Write-Host "  $_" -ForegroundColor Gray
            }
        } else {
            Write-Host "  No stats available yet" -ForegroundColor Gray
        }
    }
    catch {
        Write-Host "  Unable to retrieve stats" -ForegroundColor Gray
    }

    Write-Host "`nPLATFORM STATUS:" -ForegroundColor Yellow
    Write-Host "─────────────────────────────────────────────────────────────────────────────" -ForegroundColor Gray

    $allRunning = $Services.Values | Where-Object { $_.Status -match "Up" } | Measure-Object | Select-Object -ExpandProperty Count
    $total = $Services.Count

    if ($allRunning -eq $total -and $total -gt 0) {
        Write-Host "  Status: [✓] ALL SYSTEMS OPERATIONAL" -ForegroundColor Green
        Write-Host "  Version: 1.0.0 Production" -ForegroundColor Green
        Write-Host "  Ready for use: YES" -ForegroundColor Green
    } else {
        Write-Host "  Status: [!] PARTIAL SYSTEM" -ForegroundColor Yellow
        Write-Host "  Running: $allRunning / $total services" -ForegroundColor Yellow
    }

    Write-Host "`n" -ForegroundColor Gray
    Write-Host "Press Ctrl+C to stop monitoring..." -ForegroundColor Gray
    Write-Host "Auto-refresh in $RefreshInterval seconds..." -ForegroundColor Gray
}

# Main monitoring loop
$iteration = 0

try {
    while ($true) {
        $iteration++

        # Get current service status
        $services = Get-ServiceStatus

        # Display dashboard
        Format-Dashboard -Services $services -IterationCount $iteration

        # Wait for next refresh
        Start-Sleep -Seconds $RefreshInterval
    }
}
catch [System.OperationCanceledException] {
    Write-Host "`n`nMonitoring stopped by user." -ForegroundColor Yellow
}
finally {
    Write-Host "`nMonitoring dashboard closed." -ForegroundColor Gray
}
