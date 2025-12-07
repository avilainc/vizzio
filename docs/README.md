# Vizzio Platform Documentation

Welcome to the Vizzio Platform documentation. This directory contains comprehensive guides, specifications, and playbooks for the unified API surface, observability, and partner integrations.

## 📚 Documentation Index

### Architecture & Design
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Complete platform architecture including API gateway, telemetry pipeline, and service design
- **[openapi.yaml](openapi.yaml)** - OpenAPI 3.0 specification for the unified API surface

### API Testing & Monitoring
- **[postman-collection.json](postman-collection.json)** - Postman collection with automated tests for all API endpoints
- **[grafana-dashboard.json](grafana-dashboard.json)** - Pre-configured Grafana dashboard for monitoring platform health

### Operations & Incident Response
- **[INCIDENT_RESPONSE_PLAYBOOK.md](INCIDENT_RESPONSE_PLAYBOOK.md)** - Step-by-step playbooks for handling incidents and outages
- **[PARTNER_INTEGRATIONS.md](PARTNER_INTEGRATIONS.md)** - Setup and configuration guides for Stripe, email, and GitHub App integrations

### GitHub Marketplace
- **[github-app-manifest.json](github-app-manifest.json)** - GitHub App manifest for marketplace submission

## 🚀 Quick Start

### For Developers

1. **Review Architecture**: Start with [ARCHITECTURE.md](ARCHITECTURE.md) to understand the system design
2. **API Reference**: Use [openapi.yaml](openapi.yaml) for API contracts and [postman-collection.json](postman-collection.json) for testing
3. **Local Development**: Follow setup instructions in project root README

### For Operations

1. **Setup Monitoring**: Import [grafana-dashboard.json](grafana-dashboard.json) to your Grafana instance
2. **Configure Alerts**: Set up alerting based on metrics defined in architecture doc
3. **Incident Response**: Familiarize yourself with [INCIDENT_RESPONSE_PLAYBOOK.md](INCIDENT_RESPONSE_PLAYBOOK.md)

### For Integration Partners

1. **Partner Setup**: Follow [PARTNER_INTEGRATIONS.md](PARTNER_INTEGRATIONS.md) for Stripe, email, and GitHub App setup
2. **API Access**: Use the OpenAPI spec to understand available endpoints
3. **Testing**: Import Postman collection for easy API testing

## 🔧 API Gateway

The platform uses **avx-gateway** (Rust-based) as the unified API gateway with the following capabilities:

- **Routing**: Dynamic route configuration with load balancing
- **Authentication**: JWT, API keys, OAuth2
- **Rate Limiting**: Token bucket and sliding window algorithms
- **Observability**: Full OpenTelemetry integration
- **Resilience**: Circuit breakers, retries, timeouts

### Endpoints

- **Health**: `GET /health` - Service health status
- **Liveness**: `GET /liveness` - Kubernetes liveness probe
- **Readiness**: `GET /readiness` - Kubernetes readiness probe
- **Metrics**: `GET /metrics` - Prometheus-compatible metrics

See [openapi.yaml](openapi.yaml) for complete API documentation.

## 📊 Telemetry & Observability

The platform uses **avila-telemetry** for comprehensive observability:

### Golden Signals
- **Latency**: p50, p95, p99 response times
- **Traffic**: Request rate and active connections
- **Errors**: Error rate and error budget tracking
- **Saturation**: CPU, memory, disk, network usage

### Monitoring Tools
- **Grafana**: Pre-configured dashboards for visualization
- **Prometheus**: Metrics collection and alerting
- **OpenTelemetry**: Distributed tracing
- **Postman Monitors**: API uptime monitoring

### Error Budget
- **SLO**: 99.9% uptime (43.2 minutes downtime/month)
- **Tracking**: Real-time error budget consumption
- **Policy**: Freeze deployments when budget is exhausted

## 📧 Automated Reporting

### Weekly Executive Report
Automated report sent every Monday at 8:00 AM UTC containing:
- Revenue summary (from Stripe)
- Usage statistics (DAU, WAU, MAU)
- Top 10 errors and error trends
- Roadmap progress and sprint velocity

**Workflow**: `.github/workflows/weekly-executive-report.yml`

### Partner Notifications
Automated notifications for code changes and releases:
- Push events on main/develop branches
- Pull request events
- Issue creation
- Release publication

**Workflow**: `.github/workflows/notify-partners.yml`

## 🔐 Partner Integrations

### Stripe (Billing)
- Subscription management
- Invoice processing
- Webhook handling
- Revenue reporting

**Setup Guide**: [PARTNER_INTEGRATIONS.md#stripe-integration](PARTNER_INTEGRATIONS.md#stripe-integration)

### Email (SMTP)
- Notification delivery
- Executive reports
- Alert emails
- Partner communications

**Setup Guide**: [PARTNER_INTEGRATIONS.md#email-integration-smtp](PARTNER_INTEGRATIONS.md#email-integration-smtp)

### GitHub App
- Repository insights
- Issue/PR tracking
- Code quality metrics
- Deployment tracking

**Setup Guide**: [PARTNER_INTEGRATIONS.md#github-app-integration](PARTNER_INTEGRATIONS.md#github-app-integration)

## 🧪 Testing & Validation

### Contract Testing
Automated contract testing ensures API compatibility:
- OpenAPI schema validation
- Breaking change detection
- Postman collection tests

**Workflow**: `.github/workflows/api-contract-testing.yml`

### Test Environments
- **Production**: `https://api.vizzio.com/v1`
- **Staging**: `https://staging-api.vizzio.com/v1`
- **Local**: `http://localhost:8080/v1`

### Running Tests Locally
```bash
# Install Newman (Postman CLI)
npm install -g newman

# Run collection against local API
newman run docs/postman-collection.json \
  --environment <(echo '{"values":[{"key":"base_url","value":"http://localhost:8080/v1"}]}')
```

## 🚨 Incident Response

### Severity Levels
- **P0 - Critical**: 15-minute response time
- **P1 - High**: 1-hour response time
- **P2 - Medium**: 4-hour response time
- **P3 - Low**: 1-business-day response time

### Common Scenarios
- API Gateway failures
- Database connection issues
- High error rates
- Performance degradation
- Partner integration failures

**Full Playbook**: [INCIDENT_RESPONSE_PLAYBOOK.md](INCIDENT_RESPONSE_PLAYBOOK.md)

## 📈 Performance Targets

### API Gateway
- **Throughput**: 10,000 requests/second
- **Latency**: p99 < 100ms
- **Uptime**: 99.9%

### Backend Services
- **Response Time**: p95 < 200ms
- **Error Rate**: < 0.1%
- **Availability**: 99.9%

## 🔄 CI/CD Pipeline

### Workflows
1. **API Contract Testing**: Validates OpenAPI spec and runs contract tests
2. **Weekly Executive Report**: Generates and sends weekly report to stakeholders
3. **Partner Notifications**: Sends notifications on repository events

### Deployment Validation
- Health check verification
- Smoke tests for critical paths
- Automatic rollback on failure

## 📝 Documentation Standards

When contributing to documentation:

1. **Keep it Updated**: Update docs with code changes
2. **Use Examples**: Provide code examples and curl commands
3. **Link References**: Cross-reference related documents
4. **Version Control**: Note version and last update date
5. **Clear Structure**: Use headings, lists, and tables

## 🔗 Related Resources

### Internal Links
- [Main Repository README](../README.md)
- [Backend Service Documentation](../packages/client-strategy-analyzer/backend/README.md)
- [Gateway Implementation](../packages/avx/avx-gateway/src/lib.rs)
- [Telemetry Library](../packages/avila/avila-telemetry/src/lib.rs)

### External References
- [OpenTelemetry Specification](https://opentelemetry.io/docs/)
- [Google SRE Book](https://sre.google/books/)
- [Stripe API Documentation](https://stripe.com/docs/api)
- [GitHub Apps Documentation](https://docs.github.com/en/apps)
- [Postman Learning Center](https://learning.postman.com/)

## 🤝 Contributing

To contribute to the documentation:

1. Create a branch: `git checkout -b docs/your-topic`
2. Make changes following the standards above
3. Test any code examples
4. Submit a pull request
5. Request review from the platform team

## 📞 Support

For questions or assistance:

- **Platform Team**: Slack #platform-team
- **On-Call Engineer**: PagerDuty
- **General Support**: platform@vizzio.com

---

**Last Updated**: 2024-12-07  
**Maintained By**: Vizzio Platform Team  
**Version**: 1.0.0
