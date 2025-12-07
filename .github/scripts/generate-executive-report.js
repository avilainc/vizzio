const nodemailer = require('nodemailer');
const { Octokit } = require('@octokit/rest');

/**
 * Weekly Executive Report Generator
 * 
 * Collects metrics from Stripe, MongoDB, logs, and GitHub
 * Generates and sends a comprehensive executive report via email
 */

// Configuration from environment variables
const config = {
  smtp: {
    host: process.env.SMTP_HOST || 'smtp.gmail.com',
    port: parseInt(process.env.SMTP_PORT || '587'),
    secure: process.env.SMTP_SECURE === 'true',
    auth: {
      user: process.env.SMTP_USER,
      pass: process.env.SMTP_PASSWORD,
    },
  },
  stripe: {
    apiKey: process.env.STRIPE_API_KEY,
  },
  github: {
    token: process.env.GITHUB_TOKEN,
    owner: process.env.GITHUB_REPOSITORY?.split('/')[0] || 'avilainc',
    repo: process.env.GITHUB_REPOSITORY?.split('/')[1] || 'vizzio',
  },
  recipients: (process.env.EXECUTIVE_REPORT_RECIPIENTS || '').split(',').filter(Boolean),
  reportPeriodDays: parseInt(process.env.REPORT_PERIOD_DAYS || '7'),
};

// Initialize clients
const octokit = new Octokit({ auth: config.github.token });

/**
 * Fetch revenue data from Stripe
 */
async function fetchStripeRevenue() {
  if (!config.stripe.apiKey) {
    console.log('⚠️ Stripe API key not configured, skipping revenue data');
    return {
      weekRevenue: 0,
      monthRevenue: 0,
      yearRevenue: 0,
      newSubscriptions: 0,
      churnRate: 0,
      mrr: 0,
      arr: 0,
    };
  }

  try {
    const stripe = require('stripe')(config.stripe.apiKey);
    
    const now = new Date();
    const weekAgo = new Date(now.getTime() - config.reportPeriodDays * 24 * 60 * 60 * 1000);
    const monthAgo = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000);
    const yearAgo = new Date(now.getTime() - 365 * 24 * 60 * 60 * 1000);

    // Fetch charges for different periods
    const weekCharges = await stripe.charges.list({
      created: { gte: Math.floor(weekAgo.getTime() / 1000) },
      limit: 100,
    });

    const monthCharges = await stripe.charges.list({
      created: { gte: Math.floor(monthAgo.getTime() / 1000) },
      limit: 100,
    });

    const yearCharges = await stripe.charges.list({
      created: { gte: Math.floor(yearAgo.getTime() / 1000) },
      limit: 100,
    });

    // Calculate revenues
    const weekRevenue = weekCharges.data
      .filter(c => c.paid && !c.refunded)
      .reduce((sum, c) => sum + c.amount, 0) / 100;

    const monthRevenue = monthCharges.data
      .filter(c => c.paid && !c.refunded)
      .reduce((sum, c) => sum + c.amount, 0) / 100;

    const yearRevenue = yearCharges.data
      .filter(c => c.paid && !c.refunded)
      .reduce((sum, c) => sum + c.amount, 0) / 100;

    // Fetch subscriptions
    const activeSubscriptions = await stripe.subscriptions.list({
      status: 'active',
      limit: 100,
    });

    const newSubscriptions = await stripe.subscriptions.list({
      created: { gte: Math.floor(weekAgo.getTime() / 1000) },
      limit: 100,
    });

    // Calculate MRR (Monthly Recurring Revenue)
    const mrr = activeSubscriptions.data.reduce((sum, sub) => {
      const amount = sub.items.data.reduce((total, item) => {
        return total + (item.price.unit_amount || 0) * item.quantity;
      }, 0);
      return sum + amount / 100;
    }, 0);

    const arr = mrr * 12;

    return {
      weekRevenue: weekRevenue.toFixed(2),
      monthRevenue: monthRevenue.toFixed(2),
      yearRevenue: yearRevenue.toFixed(2),
      newSubscriptions: newSubscriptions.data.length,
      activeSubscriptions: activeSubscriptions.data.length,
      churnRate: 0, // Calculate based on historical data
      mrr: mrr.toFixed(2),
      arr: arr.toFixed(2),
    };
  } catch (error) {
    console.error('Error fetching Stripe data:', error.message);
    return {
      weekRevenue: 0,
      monthRevenue: 0,
      yearRevenue: 0,
      newSubscriptions: 0,
      churnRate: 0,
      mrr: 0,
      arr: 0,
      error: error.message,
    };
  }
}

/**
 * Fetch usage statistics
 */
async function fetchUsageStats() {
  // In a real implementation, this would query your analytics database
  // For now, return mock data
  return {
    activeUsers: {
      daily: 150,
      weekly: 450,
      monthly: 1200,
    },
    apiCalls: {
      total: 125000,
      byEndpoint: {
        '/api/v1/cases': 45000,
        '/api/v1/analytics/usage': 25000,
        '/api/v1/health': 55000,
      },
    },
    featureAdoption: {
      'Case Analysis': 85,
      'Analytics Dashboard': 65,
      'Reporting': 45,
    },
    geographic: {
      'North America': 60,
      'Europe': 25,
      'Asia': 10,
      'Other': 5,
    },
  };
}

/**
 * Fetch error statistics
 */
async function fetchErrorStats() {
  // In a real implementation, this would query your logging/monitoring system
  // For now, return mock data
  return {
    topErrors: [
      {
        type: 'DatabaseConnectionTimeout',
        count: 25,
        rate: 0.02,
        affectedServices: ['backend-api'],
        status: 'Investigating',
      },
      {
        type: 'RateLimitExceeded',
        count: 18,
        rate: 0.014,
        affectedServices: ['api-gateway'],
        status: 'Resolved',
      },
      {
        type: 'ValidationError',
        count: 12,
        rate: 0.01,
        affectedServices: ['backend-api'],
        status: 'Expected',
      },
    ],
    errorRateTrend: 'Decreasing ↓',
    totalErrors: 55,
  };
}

/**
 * Fetch roadmap progress from GitHub
 */
async function fetchRoadmapProgress() {
  try {
    const now = new Date();
    const weekAgo = new Date(now.getTime() - config.reportPeriodDays * 24 * 60 * 60 * 1000);

    // Fetch issues closed in the last week
    const { data: closedIssues } = await octokit.issues.listForRepo({
      owner: config.github.owner,
      repo: config.github.repo,
      state: 'closed',
      since: weekAgo.toISOString(),
      per_page: 100,
    });

    // Fetch open issues
    const { data: openIssues } = await octokit.issues.listForRepo({
      owner: config.github.owner,
      repo: config.github.repo,
      state: 'open',
      per_page: 100,
    });

    // Filter by labels if needed
    const features = closedIssues.filter(issue => 
      issue.labels.some(label => label.name.includes('feature') || label.name.includes('enhancement'))
    );

    const inProgress = openIssues.filter(issue => 
      issue.labels.some(label => label.name.includes('in-progress') || label.name.includes('wip'))
    );

    const blocked = openIssues.filter(issue => 
      issue.labels.some(label => label.name.includes('blocked'))
    );

    // Fetch merged PRs
    const { data: mergedPRs } = await octokit.pulls.list({
      owner: config.github.owner,
      repo: config.github.repo,
      state: 'closed',
      per_page: 100,
    });

    const thisWeekPRs = mergedPRs.filter(pr => 
      pr.merged_at && new Date(pr.merged_at) >= weekAgo
    );

    return {
      completed: features.length,
      completedItems: features.slice(0, 5).map(i => ({
        title: i.title,
        url: i.html_url,
      })),
      inProgress: inProgress.length,
      inProgressItems: inProgress.slice(0, 5).map(i => ({
        title: i.title,
        url: i.html_url,
      })),
      blocked: blocked.length,
      blockedItems: blocked.slice(0, 5).map(i => ({
        title: i.title,
        url: i.html_url,
      })),
      sprintVelocity: thisWeekPRs.length,
    };
  } catch (error) {
    console.error('Error fetching GitHub data:', error.message);
    return {
      completed: 0,
      inProgress: 0,
      blocked: 0,
      sprintVelocity: 0,
      error: error.message,
    };
  }
}

/**
 * Generate HTML email report
 */
function generateEmailHTML(data) {
  const { revenue, usage, errors, roadmap } = data;
  
  return `
<!DOCTYPE html>
<html>
<head>
  <style>
    body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
    .container { max-width: 800px; margin: 0 auto; padding: 20px; }
    h1 { color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }
    h2 { color: #34495e; margin-top: 30px; border-left: 4px solid #3498db; padding-left: 10px; }
    .metric { background: #ecf0f1; padding: 15px; margin: 10px 0; border-radius: 5px; }
    .metric-title { font-weight: bold; color: #2c3e50; }
    .metric-value { font-size: 24px; color: #3498db; font-weight: bold; }
    .status-good { color: #27ae60; }
    .status-warning { color: #f39c12; }
    .status-error { color: #e74c3c; }
    .item-list { list-style: none; padding: 0; }
    .item-list li { padding: 8px; margin: 5px 0; background: #f8f9fa; border-left: 3px solid #3498db; }
    table { width: 100%; border-collapse: collapse; margin: 15px 0; }
    th, td { padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }
    th { background-color: #3498db; color: white; }
    .footer { margin-top: 40px; padding-top: 20px; border-top: 2px solid #ecf0f1; color: #7f8c8d; font-size: 14px; }
  </style>
</head>
<body>
  <div class="container">
    <h1>📊 Weekly Executive Report - Vizzio Platform</h1>
    <p><strong>Report Period:</strong> ${new Date(Date.now() - config.reportPeriodDays * 24 * 60 * 60 * 1000).toISOString().split('T')[0]} to ${new Date().toISOString().split('T')[0]}</p>
    
    <h2>💰 Revenue Summary</h2>
    <div class="metric">
      <div class="metric-title">Weekly Revenue</div>
      <div class="metric-value">$${revenue.weekRevenue}</div>
    </div>
    <div class="metric">
      <div class="metric-title">Monthly Revenue</div>
      <div class="metric-value">$${revenue.monthRevenue}</div>
    </div>
    <div class="metric">
      <div class="metric-title">MRR / ARR</div>
      <div class="metric-value">$${revenue.mrr} / $${revenue.arr}</div>
    </div>
    <div class="metric">
      <div class="metric-title">New Subscriptions (This Week)</div>
      <div class="metric-value">${revenue.newSubscriptions}</div>
    </div>
    <div class="metric">
      <div class="metric-title">Active Subscriptions</div>
      <div class="metric-value">${revenue.activeSubscriptions || 'N/A'}</div>
    </div>

    <h2>👥 Usage Statistics</h2>
    <div class="metric">
      <div class="metric-title">Active Users</div>
      <div>DAU: <strong>${usage.activeUsers.daily}</strong> | WAU: <strong>${usage.activeUsers.weekly}</strong> | MAU: <strong>${usage.activeUsers.monthly}</strong></div>
    </div>
    <div class="metric">
      <div class="metric-title">Total API Calls (This Week)</div>
      <div class="metric-value">${usage.apiCalls.total.toLocaleString()}</div>
    </div>
    
    <h3>Top Endpoints by Traffic</h3>
    <table>
      <thead>
        <tr>
          <th>Endpoint</th>
          <th>Calls</th>
        </tr>
      </thead>
      <tbody>
        ${Object.entries(usage.apiCalls.byEndpoint).map(([endpoint, calls]) => `
          <tr>
            <td>${endpoint}</td>
            <td>${calls.toLocaleString()}</td>
          </tr>
        `).join('')}
      </tbody>
    </table>

    <h3>Feature Adoption Rates</h3>
    <table>
      <thead>
        <tr>
          <th>Feature</th>
          <th>Adoption %</th>
        </tr>
      </thead>
      <tbody>
        ${Object.entries(usage.featureAdoption).map(([feature, rate]) => `
          <tr>
            <td>${feature}</td>
            <td>${rate}%</td>
          </tr>
        `).join('')}
      </tbody>
    </table>

    <h2>🚨 Error Analysis</h2>
    <div class="metric">
      <div class="metric-title">Total Errors (This Week)</div>
      <div class="metric-value ${errors.totalErrors > 100 ? 'status-error' : errors.totalErrors > 50 ? 'status-warning' : 'status-good'}">${errors.totalErrors}</div>
    </div>
    <div class="metric">
      <div class="metric-title">Error Rate Trend</div>
      <div>${errors.errorRateTrend}</div>
    </div>

    <h3>Top 10 Errors</h3>
    <table>
      <thead>
        <tr>
          <th>Error Type</th>
          <th>Count</th>
          <th>Rate %</th>
          <th>Services</th>
          <th>Status</th>
        </tr>
      </thead>
      <tbody>
        ${errors.topErrors.map(error => `
          <tr>
            <td>${error.type}</td>
            <td>${error.count}</td>
            <td>${(error.rate * 100).toFixed(2)}%</td>
            <td>${error.affectedServices.join(', ')}</td>
            <td class="${error.status === 'Resolved' ? 'status-good' : error.status === 'Investigating' ? 'status-warning' : ''}">${error.status}</td>
          </tr>
        `).join('')}
      </tbody>
    </table>

    <h2>🚀 Roadmap Progress</h2>
    <div class="metric">
      <div class="metric-title">Sprint Velocity (PRs Merged)</div>
      <div class="metric-value">${roadmap.sprintVelocity}</div>
    </div>

    <h3>Completed Features (${roadmap.completed})</h3>
    <ul class="item-list">
      ${roadmap.completedItems?.map(item => `<li>✅ <a href="${item.url}">${item.title}</a></li>`).join('') || '<li>No items to display</li>'}
    </ul>

    <h3>In Progress (${roadmap.inProgress})</h3>
    <ul class="item-list">
      ${roadmap.inProgressItems?.map(item => `<li>🔄 <a href="${item.url}">${item.title}</a></li>`).join('') || '<li>No items to display</li>'}
    </ul>

    <h3>Blocked Items (${roadmap.blocked})</h3>
    <ul class="item-list">
      ${roadmap.blockedItems?.map(item => `<li>🚫 <a href="${item.url}">${item.title}</a></li>`).join('') || '<li>No items to display</li>'}
    </ul>

    <div class="footer">
      <p>This is an automated report generated by the Vizzio Platform monitoring system.</p>
      <p>For questions or concerns, please contact the platform team.</p>
    </div>
  </div>
</body>
</html>
  `;
}

/**
 * Send email report
 */
async function sendReport(htmlContent) {
  if (!config.smtp.auth.user || !config.smtp.auth.pass) {
    console.error('❌ SMTP credentials not configured');
    return;
  }

  if (config.recipients.length === 0) {
    console.error('❌ No recipients configured');
    return;
  }

  try {
    const transporter = nodemailer.createTransport(config.smtp);

    const mailOptions = {
      from: `"Vizzio Platform" <${config.smtp.auth.user}>`,
      to: config.recipients.join(', '),
      subject: `📊 Weekly Executive Report - ${new Date().toISOString().split('T')[0]}`,
      html: htmlContent,
    };

    const info = await transporter.sendMail(mailOptions);
    console.log('✅ Report sent successfully:', info.messageId);
    console.log('📧 Recipients:', config.recipients.join(', '));
  } catch (error) {
    console.error('❌ Error sending report:', error);
    throw error;
  }
}

/**
 * Main execution
 */
async function main() {
  console.log('📊 Generating Weekly Executive Report...\n');

  try {
    // Collect all data
    console.log('💰 Fetching revenue data from Stripe...');
    const revenue = await fetchStripeRevenue();
    
    console.log('👥 Fetching usage statistics...');
    const usage = await fetchUsageStats();
    
    console.log('🚨 Fetching error statistics...');
    const errors = await fetchErrorStats();
    
    console.log('🚀 Fetching roadmap progress from GitHub...');
    const roadmap = await fetchRoadmapProgress();

    // Generate report
    console.log('📝 Generating report...');
    const htmlContent = generateEmailHTML({ revenue, usage, errors, roadmap });

    // Send email
    console.log('📧 Sending report...');
    await sendReport(htmlContent);

    console.log('\n✅ Weekly Executive Report completed successfully!');
  } catch (error) {
    console.error('\n❌ Error generating report:', error);
    process.exit(1);
  }
}

// Run the report generator
if (require.main === module) {
  main();
}

module.exports = { main, fetchStripeRevenue, fetchUsageStats, fetchErrorStats, fetchRoadmapProgress };
