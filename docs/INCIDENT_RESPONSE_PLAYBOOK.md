# Incident Response Playbook

## Overview

This playbook provides step-by-step procedures for responding to incidents in the Vizzio platform.

## Severity Levels

| Level | Response Time | Description |
|-------|---------------|-------------|
| **P0 - Critical** | 15 minutes | Complete service outage, data loss, security breach |
| **P1 - High** | 1 hour | Major feature unavailable, significant performance degradation |
| **P2 - Medium** | 4 hours | Minor feature issue, isolated errors |
| **P3 - Low** | 1 business day | Cosmetic issues, documentation errors |

## General Incident Response Process

### 1. Detection (0-5 minutes)
- Alert received via PagerDuty, Slack, or monitoring system
- Acknowledge alert immediately
- Create incident ticket in issue tracker

### 2. Triage (5-15 minutes)
- Assess severity level
- Identify affected services and users
- Notify stakeholders based on severity
- Assemble response team if needed

### 3. Investigation (15-60 minutes)
- Check monitoring dashboards
- Review recent deployments and changes
- Examine logs and traces
- Identify root cause

### 4. Mitigation (varies)
- Implement immediate fix or workaround
- Roll back recent deployment if needed
- Scale resources if necessary
- Communicate status updates

### 5. Resolution
- Verify fix in production
- Monitor for recurrence
- Update incident ticket
- Notify stakeholders of resolution

### 6. Post-Mortem (within 48 hours)
- Document timeline and root cause
- Identify prevention measures
- Update runbooks and documentation
- Schedule follow-up tasks

## Specific Incident Scenarios

### API Gateway Failure

**Symptoms:**
- 502/503 errors from gateway
- Increased response times
- Health checks failing

**Investigation Steps:**
1. Check gateway pod/container status
   ```bash
   kubectl get pods -n vizzio-gateway
   kubectl logs -n vizzio-gateway <pod-name>
   ```

2. Check upstream service health
   ```bash
   curl http://backend:3000/health
   ```

3. Review gateway metrics
   - Check Grafana dashboard: "API Gateway Performance"
   - Look for CPU/memory spikes
   - Check connection pool exhaustion

**Mitigation:**
1. **Quick Fix:** Restart gateway pods
   ```bash
   kubectl rollout restart deployment/api-gateway -n vizzio-gateway
   ```

2. **Scale Up:** If resource constrained
   ```bash
   kubectl scale deployment/api-gateway --replicas=5 -n vizzio-gateway
   ```

3. **Route Around:** If specific backend is failing
   - Update gateway config to remove failing upstream
   - Deploy configuration change

4. **Circuit Breaker:** Should automatically open if configured
   - Verify circuit breaker status
   - Manually trigger if needed

**Prevention:**
- Implement auto-scaling based on CPU/memory
- Set up proper circuit breakers
- Regular load testing
- Monitor connection pool usage

---

### Database Connection Issues

**Symptoms:**
- "Connection timeout" errors
- Slow query responses
- Connection pool exhausted

**Investigation Steps:**
1. Check database status
   ```bash
   # For MongoDB
   mongosh --eval "db.serverStatus().connections"
   
   # Check connection count
   mongosh --eval "db.currentOp({active: true}).inprog.length"
   ```

2. Check application connection pool
   ```bash
   # Review application logs
   kubectl logs -n vizzio-backend <pod-name> | grep -i "connection"
   ```

3. Review slow queries
   ```bash
   # MongoDB slow query log
   mongosh --eval "db.system.profile.find({millis: {$gt: 1000}}).sort({ts: -1})"
   ```

**Mitigation:**
1. **Increase connection pool size** (temporary)
   - Update environment variable: `DB_POOL_SIZE=50`
   - Restart application

2. **Kill long-running queries**
   ```bash
   mongosh --eval "db.currentOp({secs_running: {$gte: 60}}).inprog.forEach(op => db.killOp(op.opid))"
   ```

3. **Scale database** (if resource constrained)
   - Increase instance size
   - Add read replicas

4. **Implement query optimization**
   - Add missing indexes
   - Rewrite inefficient queries

**Prevention:**
- Set up connection pool monitoring
- Implement query timeout limits
- Regular database performance reviews
- Proper indexing strategy

---

### High Error Rate

**Symptoms:**
- Error budget consumption alerts
- Spike in 5xx errors
- Increased error metrics

**Investigation Steps:**
1. Check error dashboard
   - Navigate to Grafana: "Error Tracking"
   - Identify error types and patterns

2. Review application logs
   ```bash
   # Recent errors
   kubectl logs -n vizzio-backend <pod-name> --tail=100 | grep ERROR
   ```

3. Check recent deployments
   ```bash
   kubectl rollout history deployment/backend -n vizzio-backend
   ```

4. Analyze error traces
   - Check OpenTelemetry traces for error patterns
   - Identify failing service or endpoint

**Mitigation:**
1. **Rollback deployment** (if recent change)
   ```bash
   kubectl rollout undo deployment/backend -n vizzio-backend
   ```

2. **Disable problematic feature** (feature flag)
   - Update feature flag configuration
   - Deploy config change

3. **Rate limit affected endpoint**
   - Update gateway rate limiting config
   - Reduce load on failing service

4. **Scale resources** (if capacity issue)
   ```bash
   kubectl scale deployment/backend --replicas=10 -n vizzio-backend
   ```

**Prevention:**
- Implement comprehensive testing
- Use canary deployments
- Set up error rate alerts
- Regular code quality reviews

---

### Performance Degradation

**Symptoms:**
- Increased response times (p95, p99)
- Timeout errors
- User complaints about slowness

**Investigation Steps:**
1. Check latency metrics
   - Grafana dashboard: "API Performance"
   - Identify slow endpoints

2. Review traces for slow requests
   - OpenTelemetry trace analysis
   - Identify bottlenecks (DB, external API, compute)

3. Check resource utilization
   ```bash
   # CPU and memory
   kubectl top pods -n vizzio-backend
   ```

4. Database query performance
   ```bash
   # Slow queries
   mongosh --eval "db.system.profile.find().sort({ts: -1}).limit(10)"
   ```

**Mitigation:**
1. **Scale horizontally** (if CPU bound)
   ```bash
   kubectl scale deployment/backend --replicas=8 -n vizzio-backend
   ```

2. **Optimize hot path** (if specific endpoint)
   - Add caching layer
   - Optimize database queries
   - Add indexes

3. **Rate limiting** (if traffic spike)
   - Implement rate limiting on slow endpoints
   - Prioritize premium users

4. **Circuit breaking** (if external dependency slow)
   - Configure circuit breaker for external services
   - Implement timeout and retry policies

**Prevention:**
- Regular performance testing
- Query optimization
- Caching strategy
- Auto-scaling configuration

---

### Stripe Integration Failure

**Symptoms:**
- Payment processing errors
- Webhook failures
- Subscription sync issues

**Investigation Steps:**
1. Check Stripe dashboard
   - Review recent events
   - Check webhook delivery status

2. Review webhook logs
   ```bash
   kubectl logs -n vizzio-backend <pod-name> | grep "stripe-webhook"
   ```

3. Verify Stripe API connectivity
   ```bash
   # Use environment variable instead of inline secret
   export STRIPE_API_KEY="your_key_here"
   curl -u "$STRIPE_API_KEY:" https://api.stripe.com/v1/charges
   ```

4. Check webhook endpoint
   ```bash
   curl -X POST https://api.vizzio.com/v1/billing/webhooks/stripe \
     -H "Content-Type: application/json" \
     -d '{"test": true}'
   ```

**Mitigation:**
1. **Retry failed webhooks**
   - Use Stripe Dashboard to resend events
   - Or implement manual reconciliation

2. **Check webhook signing secret**
   - Verify `STRIPE_WEBHOOK_SECRET` is correct
   - Rotate if compromised

3. **Scale webhook handler** (if rate limited)
   ```bash
   kubectl scale deployment/webhook-handler --replicas=3
   ```

4. **Manual sync** (as last resort)
   - Run reconciliation job
   - Sync subscription status from Stripe

**Prevention:**
- Monitor webhook success rate
- Implement idempotency keys
- Set up Stripe alert monitors
- Regular reconciliation jobs

---

### Email Delivery Failure

**Symptoms:**
- Email not received
- SMTP errors in logs
- Bounce notifications

**Investigation Steps:**
1. Check email service logs
   ```bash
   kubectl logs -n vizzio-backend <pod-name> | grep "email"
   ```

2. Verify SMTP configuration
   ```bash
   # Test SMTP connection
   telnet $SMTP_HOST $SMTP_PORT
   ```

3. Check email queue
   - Review pending emails
   - Check for bounce/reject messages

4. Verify SPF/DKIM/DMARC records
   ```bash
   dig TXT vizzio.com
   dig TXT _dmarc.vizzio.com
   ```

**Mitigation:**
1. **Switch to backup SMTP** (if configured)
   - Update `SMTP_HOST` environment variable
   - Restart email service

2. **Retry failed emails**
   - Resend from queue
   - Manual retry for critical emails

3. **Use alternative delivery method**
   - Switch to SendGrid/AWS SES if available
   - Use webhooks for urgent notifications

**Prevention:**
- Configure backup SMTP provider
- Monitor email delivery metrics
- Regular DNS record validation
- Implement email queue with retries

---

## Communication Templates

### Incident Notification (P0/P1)

**Slack Message:**
```
🚨 INCIDENT ALERT - P1
Service: API Gateway
Impact: Elevated error rates
Started: 2024-12-07 10:15 UTC
Status: Investigating

War room: #incident-2024-001
Updates: Every 15 minutes
```

**Email to Stakeholders:**
```
Subject: [P1 INCIDENT] Service Degradation - API Gateway

Dear Stakeholders,

We are currently experiencing elevated error rates on our API Gateway affecting approximately 20% of requests.

IMPACT: Users may experience intermittent errors when accessing the platform
START TIME: 2024-12-07 10:15 UTC
STATUS: Our team is actively investigating and working on a resolution

We will provide updates every 15 minutes until the issue is resolved.

Next update: 10:30 UTC

Best regards,
Vizzio Platform Team
```

### Incident Resolution

**Slack Message:**
```
✅ RESOLVED - P1 Incident
Service: API Gateway
Duration: 45 minutes
Root Cause: Database connection pool exhaustion
Fix: Increased pool size and restarted services

Post-mortem: Will be published within 48 hours
```

### Post-Mortem Template

```markdown
# Post-Mortem: [Incident Title]

## Summary
Brief description of what happened

## Impact
- Duration: X hours
- Affected users: X%
- Error rate: X%
- Revenue impact: $X

## Timeline (all times UTC)
- 10:15 - Alert triggered
- 10:18 - Incident acknowledged
- 10:25 - Root cause identified
- 10:40 - Fix deployed
- 11:00 - Incident resolved

## Root Cause
Detailed explanation of what caused the incident

## Resolution
How the incident was resolved

## Prevention Measures
1. Immediate actions taken
2. Short-term improvements (1-2 weeks)
3. Long-term improvements (1-3 months)

## Action Items
- [ ] Item 1 - Owner - Due date
- [ ] Item 2 - Owner - Due date

## Lessons Learned
What went well and what could be improved
```

## Emergency Contacts

| Role | Contact | Escalation Time |
|------|---------|----------------|
| On-Call Engineer | PagerDuty | Immediate |
| Engineering Lead | Slack DM | 15 minutes |
| CTO | Phone | 30 minutes (P0 only) |
| Customer Success | Email | P1+ incidents |

## Tools and Resources

- **Monitoring:** https://grafana.vizzio.com
- **Logs:** https://logs.vizzio.com
- **Traces:** https://traces.vizzio.com
- **Status Page:** https://status.vizzio.com
- **Runbooks:** /docs/runbooks/
- **Architecture:** /docs/ARCHITECTURE.md

## Training

All engineers should complete:
1. Incident response simulation (quarterly)
2. Monitoring tools training
3. Deployment procedures
4. Security incident training

---

**Document Version:** 1.0  
**Last Updated:** 2024-12-07  
**Owner:** Platform Team
