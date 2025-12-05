# VIZZIO PLATFORM - PRODUCTION DEPLOYMENT GUIDE

## 🚀 QUICK START DEPLOYMENT

### Prerequisites
- Docker & Docker Compose installed
- Node.js 18+ installed
- 4GB+ RAM available
- 10GB+ disk space

### Step 1: Configure Environment
```powershell
# Copy the production config template
Copy-Item .env.production.example .env.production

# Edit with your values
notepad .env.production

# Key values to update:
# - MONGODB_PASSWORD
# - REDIS_PASSWORD
# - RABBITMQ_PASSWORD
# - JWT_SECRET (generate strong key)
# - STRIPE_SECRET_KEY
# - OPENAI_API_KEY
# - Other external service keys
```

### Step 2: Build & Compile
```powershell
# Install dependencies
npm install

# Build all packages
npm run build

# Verify build
npm run test
```

### Step 3: Start Infrastructure
```powershell
# Start Docker services (MongoDB, Redis, RabbitMQ)
docker-compose up -d

# Verify services are running
docker-compose ps

# Check logs
docker-compose logs -f
```

### Step 4: Run Backend
```powershell
# Option A: Direct
npm start -w @vizzio/backend

# Option B: Development mode
npm run dev -w @vizzio/backend
```

### Step 5: Run Frontend (in new terminal)
```powershell
# Start Next.js frontend
cd packages/frontend
npm start

# Or development mode
npm run dev
```

### Step 6: Verify Deployment
```powershell
# Check backend health
curl http://localhost:3000/health

# Check frontend
Open-Browser http://localhost:3001

# Check RabbitMQ Manager
Open-Browser http://localhost:15672
# User: admin / Pass: from .env.production
```

---

## 🐳 DOCKER PRODUCTION DEPLOYMENT

### Using docker-compose.prod.yml

```powershell
# Build images
docker-compose -f docker-compose.prod.yml build

# Start all services
docker-compose -f docker-compose.prod.yml up -d

# Check status
docker-compose -f docker-compose.prod.yml ps

# View logs
docker-compose -f docker-compose.prod.yml logs -f

# Stop services
docker-compose -f docker-compose.prod.yml down

# Stop and remove volumes
docker-compose -f docker-compose.prod.yml down -v
```

---

## 📊 MONITORING

### Docker Container Logs
```powershell
# View all logs
docker-compose logs -f

# View specific service
docker-compose logs -f backend

# View last 100 lines
docker-compose logs --tail=100

# View logs with timestamps
docker-compose logs --timestamps
```

### Health Checks
```powershell
# Backend health
curl http://localhost:3000/health

# Database connection
mongo "mongodb://admin:password123@localhost:27017/vizzio?authSource=admin"

# Redis connection
redis-cli -h localhost

# RabbitMQ status
curl -u admin:password123 http://localhost:15672/api/overview
```

---

## 🔧 COMMON OPERATIONS

### Restart Services
```powershell
# Restart all
docker-compose restart

# Restart specific service
docker-compose restart backend
docker-compose restart mongodb
```

### View Logs in Real-time
```powershell
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend

# Last N lines
docker-compose logs --tail=50 backend
```

### Execute Commands in Container
```powershell
# Backend shell
docker-compose exec backend /bin/sh

# MongoDB shell
docker-compose exec mongodb mongosh

# Redis CLI
docker-compose exec redis redis-cli
```

### Database Backup
```powershell
# Backup MongoDB
docker-compose exec mongodb mongodump --authenticationDatabase admin -u admin -p password123 -o /backup/$(Get-Date -Format 'yyyyMMdd')

# Backup Redis
docker-compose exec redis redis-cli --rdb /backup/dump.rdb
```

### Database Restore
```powershell
# Restore MongoDB
docker-compose exec -T mongodb mongorestore --authenticationDatabase admin -u admin -p password123 /backup/backup_folder
```

---

## 🔒 SECURITY CHECKLIST

- [ ] Change all default passwords in .env.production
- [ ] Generate strong JWT_SECRET (32+ characters)
- [ ] Enable SSL/TLS certificates
- [ ] Configure CORS_ORIGIN properly
- [ ] Set up firewall rules
- [ ] Enable authentication on all services
- [ ] Configure rate limiting
- [ ] Set up monitoring/alerting
- [ ] Enable database encryption
- [ ] Regular backup strategy

---

## 🚨 TROUBLESHOOTING

### Backend not starting
```powershell
# Check logs
docker-compose logs backend

# Verify port is available
netstat -ano | findstr :3000

# Check MongoDB connection
docker-compose exec backend curl mongodb:27017
```

### Database connection issues
```powershell
# Check MongoDB status
docker-compose ps mongodb

# Test connection
mongo "mongodb://admin:password123@localhost:27017/vizzio?authSource=admin"

# Check MongoDB logs
docker-compose logs mongodb
```

### Memory/Performance issues
```powershell
# Check Docker stats
docker stats

# Reduce MongoDB cache
# Edit docker-compose.yml: --wiredTigerCacheSizeGB 1

# Limit container memory in docker-compose.yml:
# mem_limit: 512m
# memswap_limit: 512m
```

### Port conflicts
```powershell
# Find process using port
netstat -ano | findstr :3000

# Kill process
taskkill /PID <PID> /F

# Or change port in docker-compose.yml
```

---

## 📈 PERFORMANCE OPTIMIZATION

### Database
```powershell
# Create indexes (in MongoDB shell)
db.workflows.createIndex({ "createdAt": -1 })
db.automations.createIndex({ "status": 1, "updatedAt": -1 })
```

### Caching Strategy
```powershell
# Redis configuration in .env.production
REDIS_MAX_TTL=3600
CACHE_STRATEGY=lru
```

### Load Balancing (Optional)
```powershell
# Behind nginx/reverse proxy
# Multiple backend instances on different ports
# Use nginx load balancing configuration
```

---

## 🔄 UPDATES & PATCHES

### Update Dependencies
```powershell
# Check for updates
npm outdated

# Update packages
npm update

# Major version updates
npm install npm@latest
```

### Update Docker Images
```powershell
# Pull latest images
docker-compose pull

# Rebuild images
docker-compose build --no-cache

# Restart services
docker-compose up -d
```

---

## 📝 DEPLOYMENT CHECKLIST

- [ ] All prerequisites installed
- [ ] .env.production configured
- [ ] npm install completed
- [ ] npm run build succeeded
- [ ] Docker services running
- [ ] Backend health check passing
- [ ] Frontend accessible
- [ ] Database connectivity verified
- [ ] API endpoints responding
- [ ] Logs showing normal operation
- [ ] Security settings configured
- [ ] Backups configured

---

## 📞 SUPPORT

For issues or questions:
1. Check DEPLOYMENT_STATUS.md
2. Review Docker logs
3. Verify .env.production settings
4. Check system resources
5. Review STARTUP_GUIDE.md

---

**Deployment Status:** ✅ Ready for Production
**Version:** 1.0.0
**Last Updated:** 2025-12-05
