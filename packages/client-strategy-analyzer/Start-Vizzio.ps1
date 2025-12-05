# Vizzio Platform - Startup Script
# This script starts all services and opens the application

Write-Host "
===============================================================
              VIZZIO PLATFORM - STARTUP SCRIPT
              Enterprise Automation Platform
===============================================================
" -ForegroundColor Cyan

# Check if Docker is running
Write-Host "`nChecking Docker status..." -ForegroundColor Yellow
try {
    docker ps | Out-Null
    Write-Host "[OK] Docker is running" -ForegroundColor Green
} catch {
    Write-Host "[ERROR] Docker is NOT running. Please start Docker Desktop." -ForegroundColor Red
    Write-Host "   After starting Docker, run this script again." -ForegroundColor Yellow
    exit 1
}

# Check if Docker Compose file exists
$dockerComposeFile = "docker-compose.yml"
if (-not (Test-Path $dockerComposeFile)) {
    Write-Host "[ERROR] docker-compose.yml not found in current directory" -ForegroundColor Red
    exit 1
}

# Start Docker services
Write-Host "`nStarting Docker Compose services..." -ForegroundColor Yellow
docker-compose up -d

if ($LASTEXITCODE -eq 0) {
    Write-Host "[OK] Docker services started successfully" -ForegroundColor Green
} else {
    Write-Host "[ERROR] Failed to start Docker services" -ForegroundColor Red
    exit 1
}

# Wait for services to be ready
Write-Host "`nWaiting for services to be ready..." -ForegroundColor Yellow
Start-Sleep -Seconds 5

# Check service status
Write-Host "`nService Status:" -ForegroundColor Cyan
docker-compose ps

# Display connection information
Write-Host "`n
===============================================================
                    ACCESS POINTS
===============================================================
  Frontend Dashboard:  http://localhost:3001
  Backend API:         http://localhost:3000
  RabbitMQ Manager:    http://localhost:15672
                       (Username: admin / Password: password123)
  MongoDB:             mongodb://localhost:27017
  Redis:               redis://localhost:6379
===============================================================
  Database Credentials:
  - MongoDB User: admin
  - MongoDB Pass: password123
  - RabbitMQ User: admin
  - RabbitMQ Pass: password123
===============================================================
" -ForegroundColor Green

# Offer to open browser
Write-Host "`nOpen browser windows?" -ForegroundColor Yellow
$response = Read-Host "Press Y to open Frontend [Y/n]"

if ($response -eq "Y" -or $response -eq "y" -or $response -eq "") {
    Write-Host "Opening http://localhost:3001..." -ForegroundColor Cyan
    Start-Process "http://localhost:3001"
}

Write-Host "`n
Vizzio Platform is now running!
View logs with: docker-compose logs -f
Stop services with: docker-compose down
" -ForegroundColor Green

# Keep the prompt open
Read-Host "Press Enter to continue monitoring..."
