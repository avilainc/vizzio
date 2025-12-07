# Partner Integrations Guide

## Overview

This guide provides comprehensive instructions for setting up and managing partner integrations in the Vizzio platform, including Stripe billing, email notifications, and GitHub App.

## Table of Contents

1. [Stripe Integration](#stripe-integration)
2. [Email Integration (SMTP)](#email-integration-smtp)
3. [GitHub App Integration](#github-app-integration)
4. [Secret Management](#secret-management)
5. [Monitoring and Troubleshooting](#monitoring-and-troubleshooting)

---

## Stripe Integration

### Purpose
Handle billing, subscriptions, invoices, and revenue tracking for the Vizzio platform.

### Setup Steps

#### 1. Create Stripe Account
1. Sign up at https://stripe.com
2. Complete business verification
3. Enable production mode after testing

#### 2. Get API Keys
1. Navigate to Developers > API keys
2. Copy the **Secret key** (starts with `sk_`)
3. Copy the **Publishable key** (starts with `pk_`)

#### 3. Configure Webhook Endpoint
1. Navigate to Developers > Webhooks
2. Click "Add endpoint"
3. Set URL: `https://api.vizzio.com/v1/billing/webhooks/stripe`
4. Select events to listen to:
   - `customer.subscription.created`
   - `customer.subscription.updated`
   - `customer.subscription.deleted`
   - `invoice.payment_succeeded`
   - `invoice.payment_failed`
   - `charge.succeeded`
   - `charge.failed`
5. Copy the **Webhook signing secret**

#### 4. Store Secrets in GitHub
```bash
# Add to GitHub repository secrets
gh secret set STRIPE_API_KEY --body "sk_live_..."
gh secret set STRIPE_WEBHOOK_SECRET --body "whsec_..."
gh secret set STRIPE_PUBLISHABLE_KEY --body "pk_live_..."
```

#### 5. Configure Application
Update environment variables in your deployment:
```yaml
env:
  - name: STRIPE_API_KEY
    valueFrom:
      secretKeyRef:
        name: stripe-secrets
        key: api-key
  - name: STRIPE_WEBHOOK_SECRET
    valueFrom:
      secretKeyRef:
        name: stripe-secrets
        key: webhook-secret
```

### Usage Examples

#### Create a Subscription
```javascript
const stripe = require('stripe')(process.env.STRIPE_API_KEY);

async function createSubscription(customerId, priceId) {
  const subscription = await stripe.subscriptions.create({
    customer: customerId,
    items: [{ price: priceId }],
    payment_behavior: 'default_incomplete',
    expand: ['latest_invoice.payment_intent'],
  });
  
  return subscription;
}
```

#### Handle Webhook
```javascript
const stripe = require('stripe')(process.env.STRIPE_API_KEY);

async function handleWebhook(req, res) {
  const sig = req.headers['stripe-signature'];
  const webhookSecret = process.env.STRIPE_WEBHOOK_SECRET;
  
  let event;
  try {
    event = stripe.webhooks.constructEvent(req.body, sig, webhookSecret);
  } catch (err) {
    return res.status(400).send(`Webhook Error: ${err.message}`);
  }
  
  // Handle the event
  switch (event.type) {
    case 'customer.subscription.created':
      const subscription = event.data.object;
      await handleSubscriptionCreated(subscription);
      break;
    
    case 'invoice.payment_succeeded':
      const invoice = event.data.object;
      await handlePaymentSuccess(invoice);
      break;
    
    case 'invoice.payment_failed':
      const failedInvoice = event.data.object;
      await handlePaymentFailure(failedInvoice);
      break;
    
    default:
      console.log(`Unhandled event type ${event.type}`);
  }
  
  res.json({ received: true });
}
```

#### Retrieve Revenue Data
```javascript
async function getRevenueData(startDate, endDate) {
  const charges = await stripe.charges.list({
    created: {
      gte: Math.floor(startDate.getTime() / 1000),
      lte: Math.floor(endDate.getTime() / 1000),
    },
    limit: 100,
  });
  
  const totalRevenue = charges.data
    .filter(charge => charge.paid && !charge.refunded)
    .reduce((sum, charge) => sum + charge.amount, 0) / 100;
  
  return {
    totalRevenue,
    transactionCount: charges.data.length,
    currency: charges.data[0]?.currency || 'usd',
  };
}
```

### Testing

#### Test Mode
1. Use test API keys (start with `sk_test_`)
2. Test credit card: `4242 4242 4242 4242`
3. Test webhook with Stripe CLI:
   ```bash
   stripe listen --forward-to localhost:3000/v1/billing/webhooks/stripe
   ```

### Monitoring
- Track webhook success rate: Monitor `/metrics` endpoint
- Set up alerts for payment failures
- Review Stripe Dashboard daily for anomalies

---

## Email Integration (SMTP)

### Purpose
Send notifications, reports, and alerts to users and stakeholders.

### Setup Steps

#### 1. Choose SMTP Provider
Recommended providers:
- **Gmail** (for testing/small scale)
- **SendGrid** (recommended for production)
- **AWS SES** (cost-effective, high volume)
- **Mailgun** (developer-friendly)

#### 2. Configure SMTP Settings

##### Option A: Gmail
1. Enable 2-factor authentication
2. Generate App Password: https://myaccount.google.com/apppasswords
3. Use settings:
   ```
   SMTP_HOST=smtp.gmail.com
   SMTP_PORT=587
   SMTP_SECURE=true  # Use TLS for secure connection
   SMTP_USER=your-email@gmail.com
   SMTP_PASSWORD=<app-password>
   ```

##### Option B: SendGrid
1. Sign up at https://sendgrid.com
2. Create API key: Settings > API Keys
3. Use settings:
   ```
   SMTP_HOST=smtp.sendgrid.net
   SMTP_PORT=587
   SMTP_SECURE=false
   SMTP_USER=apikey
   SMTP_PASSWORD=<your-api-key>
   ```

##### Option C: AWS SES
1. Set up SES in AWS Console
2. Verify domain and email addresses
3. Create SMTP credentials
4. Use settings:
   ```
   SMTP_HOST=email-smtp.<region>.amazonaws.com
   SMTP_PORT=587
   SMTP_SECURE=false
   SMTP_USER=<smtp-username>
   SMTP_PASSWORD=<smtp-password>
   ```

#### 3. Store Secrets
```bash
gh secret set SMTP_HOST --body "smtp.sendgrid.net"
gh secret set SMTP_PORT --body "587"
gh secret set SMTP_USER --body "apikey"
gh secret set SMTP_PASSWORD --body "SG.xxx..."
gh secret set SMTP_SECURE --body "false"
```

#### 4. Configure Recipients
```bash
# For executive reports
gh secret set EXECUTIVE_REPORT_RECIPIENTS --body "ceo@vizzio.com,cto@vizzio.com"

# For partner notifications
gh secret set PARTNER_1_EMAIL --body "partner1@example.com"
gh secret set PARTNER_1_NAME --body "Partner Company 1"
```

### Usage Example

```javascript
const nodemailer = require('nodemailer');

const transporter = nodemailer.createTransport({
  host: process.env.SMTP_HOST,
  port: parseInt(process.env.SMTP_PORT),
  secure: process.env.SMTP_SECURE === 'true',
  auth: {
    user: process.env.SMTP_USER,
    pass: process.env.SMTP_PASSWORD,
  },
});

async function sendEmail(to, subject, html) {
  const mailOptions = {
    from: '"Vizzio Platform" <noreply@vizzio.com>',
    to,
    subject,
    html,
  };
  
  const info = await transporter.sendMail(mailOptions);
  console.log('Email sent:', info.messageId);
  return info;
}
```

### Best Practices

1. **Rate Limiting**
   - Respect provider limits
   - Implement queue for bulk emails
   - Use exponential backoff for retries

2. **Deliverability**
   - Configure SPF, DKIM, DMARC records
   - Maintain good sender reputation
   - Monitor bounce rates

3. **Content**
   - Use responsive HTML templates
   - Include plain text alternative
   - Add unsubscribe link

4. **Security**
   - Use TLS for connections
   - Never log email content
   - Rotate SMTP credentials regularly

### DNS Configuration

Add these records to your domain:

```dns
; SPF Record
vizzio.com. IN TXT "v=spf1 include:_spf.google.com include:sendgrid.net ~all"

; DKIM Record (get from your provider)
default._domainkey.vizzio.com. IN TXT "v=DKIM1; k=rsa; p=MIGfMA0GCSqG..."

; DMARC Record
_dmarc.vizzio.com. IN TXT "v=DMARC1; p=quarantine; rua=mailto:dmarc@vizzio.com"
```

---

## GitHub App Integration

### Purpose
Provide repository insights, automation, and integration with the GitHub Marketplace.

### Setup Steps

#### 1. Create GitHub App
1. Navigate to Settings > Developer settings > GitHub Apps
2. Click "New GitHub App"
3. Fill in details:
   - **Name:** Vizzio Platform Insights
   - **Homepage URL:** https://vizzio.com
   - **Webhook URL:** https://api.vizzio.com/v1/github/webhooks
   - **Webhook secret:** Generate a secure random string

#### 2. Configure Permissions
Select appropriate permissions:

**Repository permissions:**
- Contents: Read
- Issues: Read & Write
- Pull requests: Read & Write
- Metadata: Read

**Organization permissions:**
- Members: Read

**Subscribe to events:**
- Issues
- Pull request
- Push
- Repository
- Release

#### 3. Install App
1. Install on your organization
2. Select repositories (All or specific)
3. Authorize the installation

#### 4. Get Credentials
1. Generate private key (download .pem file)
2. Note the App ID
3. Note the Installation ID

#### 5. Store Secrets
```bash
gh secret set GITHUB_APP_ID --body "123456"
gh secret set GITHUB_APP_INSTALLATION_ID --body "78910"
gh secret set GITHUB_APP_PRIVATE_KEY --body "$(cat private-key.pem)"
gh secret set GITHUB_APP_WEBHOOK_SECRET --body "your-webhook-secret"
```

### Usage Example

```javascript
const { App } = require('@octokit/app');
const { Octokit } = require('@octokit/rest');

// Initialize GitHub App
const app = new App({
  appId: process.env.GITHUB_APP_ID,
  privateKey: process.env.GITHUB_APP_PRIVATE_KEY,
});

// Get installation access token
async function getInstallationOctokit() {
  const installationId = process.env.GITHUB_APP_INSTALLATION_ID;
  const octokit = await app.getInstallationOctokit(installationId);
  return octokit;
}

// Example: Get repository insights
async function getRepositoryInsights(owner, repo) {
  const octokit = await getInstallationOctokit();
  
  // Get commits
  const { data: commits } = await octokit.repos.listCommits({
    owner,
    repo,
    per_page: 100,
  });
  
  // Get issues
  const { data: issues } = await octokit.issues.listForRepo({
    owner,
    repo,
    state: 'all',
    per_page: 100,
  });
  
  // Get pull requests
  const { data: pulls } = await octokit.pulls.list({
    owner,
    repo,
    state: 'all',
    per_page: 100,
  });
  
  return {
    commits: commits.length,
    issues: issues.filter(i => !i.pull_request).length,
    pullRequests: pulls.length,
    openIssues: issues.filter(i => i.state === 'open' && !i.pull_request).length,
  };
}

// Handle webhook
async function handleGitHubWebhook(req, res) {
  const signature = req.headers['x-hub-signature-256'];
  const event = req.headers['x-github-event'];
  
  // Verify webhook signature
  const isValid = verifyWebhookSignature(
    req.body,
    signature,
    process.env.GITHUB_APP_WEBHOOK_SECRET
  );
  
  if (!isValid) {
    return res.status(401).send('Invalid signature');
  }
  
  // Handle events
  switch (event) {
    case 'issues':
      await handleIssueEvent(req.body);
      break;
    case 'pull_request':
      await handlePullRequestEvent(req.body);
      break;
    default:
      console.log(`Unhandled event: ${event}`);
  }
  
  res.json({ ok: true });
}
```

### GitHub Marketplace Submission

#### Pre-submission Checklist
- [ ] App has a clear, descriptive name
- [ ] Homepage URL is live and functional
- [ ] Logo uploaded (200x200 px minimum)
- [ ] Detailed description (500 characters minimum)
- [ ] Screenshots of app in action (3-5 recommended)
- [ ] Pricing plan configured (free tier recommended)
- [ ] Support email configured
- [ ] Privacy policy URL
- [ ] Terms of service URL

#### Submission Process
1. Navigate to your GitHub App settings
2. Click "Make this app public"
3. Fill in Marketplace listing information
4. Submit for review
5. Wait for GitHub approval (typically 1-2 weeks)

#### Marketplace Categories
Select relevant categories:
- Developer tools
- Project management
- Code quality
- Continuous integration
- Monitoring

---

## Secret Management

### GitHub Secrets

#### Organization Secrets
```bash
# Set organization-level secrets (available to all repos)
gh secret set SECRET_NAME --org organization-name
```

#### Repository Secrets
```bash
# Set repository-level secrets
gh secret set SECRET_NAME --repo owner/repo
```

#### Environment Secrets
```bash
# Set environment-specific secrets
gh secret set SECRET_NAME --env production
```

### Best Practices

1. **Rotation Policy**
   - Rotate secrets every 90 days
   - Rotate immediately if compromised
   - Keep rotation history

2. **Access Control**
   - Limit who can view/edit secrets
   - Use organization-level secrets when possible
   - Audit secret access regularly

3. **Naming Convention**
   ```
   SERVICE_PURPOSE_TYPE
   Example: STRIPE_API_KEY, SMTP_PASSWORD
   ```

4. **Never Commit Secrets**
   - Use `.env.example` for templates
   - Add `.env` to `.gitignore`
   - Use git-secrets or similar tools

### Secret Verification

Create a verification script:
```bash
#!/bin/bash
# verify-secrets.sh

REQUIRED_SECRETS=(
  "STRIPE_API_KEY"
  "STRIPE_WEBHOOK_SECRET"
  "SMTP_HOST"
  "SMTP_USER"
  "SMTP_PASSWORD"
  "GITHUB_APP_ID"
  "GITHUB_APP_PRIVATE_KEY"
)

for secret in "${REQUIRED_SECRETS[@]}"; do
  if [ -z "${!secret}" ]; then
    echo "❌ Missing secret: $secret"
    exit 1
  fi
done

echo "✅ All required secrets are configured"
```

---

## Monitoring and Troubleshooting

### Monitoring Dashboards

#### Stripe Integration Metrics
- Webhook success rate (target: >99%)
- Payment success rate (target: >95%)
- Average transaction value
- Subscription churn rate

#### Email Integration Metrics
- Email delivery rate (target: >98%)
- Average delivery time
- Bounce rate (target: <2%)
- Spam complaint rate (target: <0.1%)

#### GitHub App Metrics
- API rate limit usage
- Webhook processing time
- Installation count
- Active users

### Common Issues

#### Stripe Webhook Failures
**Symptoms:** Webhooks not being received or processed

**Solutions:**
1. Verify webhook URL is accessible
2. Check webhook signing secret
3. Review application logs
4. Test with Stripe CLI
5. Ensure endpoint returns 200 status

#### Email Delivery Issues
**Symptoms:** Emails not received or marked as spam

**Solutions:**
1. Verify SMTP credentials
2. Check DNS records (SPF, DKIM, DMARC)
3. Review bounce messages
4. Test with mail-tester.com
5. Check provider rate limits

#### GitHub App Rate Limiting
**Symptoms:** API requests failing with 403 status

**Solutions:**
1. Check rate limit headers
2. Implement exponential backoff
3. Use conditional requests (ETags)
4. Cache responses when possible
5. Request rate limit increase if needed

### Support Resources

- **Stripe Support:** https://support.stripe.com
- **SendGrid Support:** https://support.sendgrid.com
- **GitHub Support:** https://support.github.com
- **Internal Documentation:** /docs/

---

**Document Version:** 1.0  
**Last Updated:** 2024-12-07  
**Owner:** Platform Team
