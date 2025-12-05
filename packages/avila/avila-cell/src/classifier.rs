//! Email classification, spam detection and conversation threading.

use crate::message::{Attachment, Email};
use avila_regex::Regex;

/// Default threshold for classifying as spam.
const DEFAULT_SPAM_THRESHOLD: f32 = 3.0;
/// Difference range that triggers a manual review suggestion.
const DEFAULT_REVIEW_MARGIN: f32 = 1.2;
/// Minimum score to consider suggesting review.
const DEFAULT_MIN_REVIEW_SCORE: f32 = 2.0;

/// File extensions frequently associated with malicious payloads.
const SUSPICIOUS_ATTACHMENT_EXTENSIONS: &[&str] = &[
    "exe", "scr", "bat", "cmd", "js", "vbs", "ps1", "jar", "apk", "msi", "com",
];

/// File extensions commonly seen in legitimate business emails.
const TRUSTED_ATTACHMENT_EXTENSIONS: &[&str] = &[
    "pdf", "doc", "docx", "ppt", "pptx", "xls", "xlsx", "odt", "ods", "odp",
];

/// Impact direction of a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleImpact {
    /// Indicates the rule contributes to a spam classification.
    Spam,
    /// Indicates the rule contributes to a ham (legitimate) classification.
    Ham,
}

/// Describes a matching rule triggered during classification.
#[derive(Debug, Clone)]
pub struct RuleMatch {
    /// Rule identifier.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Weight contributed by this rule.
    pub weight: f32,
    /// Impact direction (spam or ham).
    pub impact: RuleImpact,
    /// Optional pattern associated with the rule.
    pub pattern: Option<String>,
    /// Optional contextual details (e.g. offending link or attachment).
    pub details: Option<String>,
}

impl RuleMatch {
    fn new(name: impl Into<String>, description: impl Into<String>, weight: f32, impact: RuleImpact) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            weight,
            impact,
            pattern: None,
            details: None,
        }
    }

    fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

/// Result returned by the classifier with contextual information.
#[derive(Debug, Clone)]
pub struct ClassificationResult {
    /// Final class assigned to the email.
    pub class: EmailClass,
    /// Accumulated spam score.
    pub spam_score: f32,
    /// Accumulated ham score.
    pub ham_score: f32,
    /// Rules that fired during analysis.
    pub rule_matches: Vec<RuleMatch>,
    /// Suggestions to the caller (manual review, sanitisation, etc.).
    pub warnings: Vec<String>,
    /// True when the scores are close enough to suggest manual review.
    pub review_recommended: bool,
}

impl ClassificationResult {
    /// Returns true if the classifier flagged the email as spam.
    pub fn is_spam(&self) -> bool {
        matches!(self.class, EmailClass::Spam(_))
    }

    /// Returns true if a manual review is recommended.
    pub fn should_review(&self) -> bool {
        self.review_recommended
    }
}

/// Internal representation of a scoring rule.
struct Rule {
    name: String,
    description: String,
    pattern: String,
    weight: f32,
    regex: Regex,
    impact: RuleImpact,
}

impl Rule {
    fn new(name: &str, pattern: &str, description: &str, weight: f32, impact: RuleImpact) -> Self {
        let regex = Regex::new(pattern).expect("invalid regex pattern for classifier rule");
        Self {
            name: name.to_string(),
            description: description.to_string(),
            pattern: pattern.to_string(),
            weight,
            regex,
            impact,
        }
    }

    fn from_regex(name: &str, regex: Regex, description: &str, weight: f32, impact: RuleImpact) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            pattern: "<dynamic>".to_string(),
            weight,
            regex,
            impact,
        }
    }

    fn apply(&self, text: &str) -> Option<RuleMatch> {
        if self.regex.is_match(text) {
            let mut r = RuleMatch::new(&self.name, &self.description, self.weight, self.impact);
            if self.pattern != "<dynamic>" {
                r = r.with_pattern(self.pattern.clone());
            }
            Some(r)
        } else {
            None
        }
    }
}

/// Email classifier with weighted rules and heuristics.
pub struct EmailClassifier {
    spam_rules: Vec<Rule>,
    ham_rules: Vec<Rule>,
    spam_threshold: f32,
    review_margin: f32,
    min_review_score: f32,
}

impl EmailClassifier {
    /// Creates a classifier with built-in rules and thresholds.
    pub fn new() -> Self {
        Self {
            spam_rules: Self::default_spam_rules(),
            ham_rules: Self::default_ham_rules(),
            spam_threshold: DEFAULT_SPAM_THRESHOLD,
            review_margin: DEFAULT_REVIEW_MARGIN,
            min_review_score: DEFAULT_MIN_REVIEW_SCORE,
        }
    }

    /// Adjusts the spam score threshold.
    pub fn with_spam_threshold(mut self, threshold: f32) -> Self {
        self.spam_threshold = threshold.max(0.5);
        self
    }

    /// Adjusts the review margin.
    pub fn with_review_margin(mut self, margin: f32) -> Self {
        self.review_margin = margin.max(0.1);
        self
    }

    /// Classifies an email represented by subject/body pairs.
    pub fn classify(&self, subject: &str, body: &str) -> EmailClass {
        self.classify_text(subject, body).class
    }

    /// Returns a detailed classification for a subject/body pair.
    pub fn classify_text(&self, subject: &str, body: &str) -> ClassificationResult {
        let mut matches = Vec::new();
        let mut warnings = Vec::new();
        let mut spam_score = 0.0;
        let mut ham_score = 0.0;

        let text = format!("{}\n{}", subject, body);
        let text_lower = text.to_lowercase();
        let subject_lower = subject.to_lowercase();
        let body_lower = body.to_lowercase();

        for rule in &self.spam_rules {
            if let Some(m) = rule.apply(&text) {
                spam_score += m.weight;
                matches.push(m);
            }
        }

        for rule in &self.ham_rules {
            if let Some(m) = rule.apply(&text) {
                ham_score += m.weight;
                matches.push(m);
            }
        }

        // Heuristic: uppercase ratio
        let (upper_ratio, upper_count) = uppercase_ratio(&text);
        if upper_ratio > 0.45 && upper_count > 12 {
            spam_score += 1.3;
            matches.push(
                RuleMatch::new(
                    "excessive-uppercase",
                    "Texto com proporção alta de letras maiúsculas",
                    1.3,
                    RuleImpact::Spam,
                )
                .with_details(format!("Proporção {:.0}%", upper_ratio * 100.0)),
            );
        }

        // Heuristic: excessive punctuation
        let exclamation_count = text.matches('!').count();
        if exclamation_count >= 6 {
            spam_score += 1.5;
            matches.push(
                RuleMatch::new(
                    "excessive-exclamation",
                    "Excesso de pontos de exclamação",
                    1.5,
                    RuleImpact::Spam,
                )
                .with_details(format!("{} ocorrências", exclamation_count)),
            );
        } else if exclamation_count >= 3 {
            spam_score += 0.8;
            matches.push(
                RuleMatch::new(
                    "moderate-exclamation",
                    "Múltiplos pontos de exclamação",
                    0.8,
                    RuleImpact::Spam,
                )
                .with_details(format!("{} ocorrências", exclamation_count)),
            );
        }

        // Heuristic: base64 blobs (often used for hidden payloads)
        if contains_suspicious_base64(&body_lower) {
            spam_score += 1.0;
            matches.push(
                RuleMatch::new(
                    "embedded-base64",
                    "Bloco de texto semelhante a Base64 encontrado",
                    1.0,
                    RuleImpact::Spam,
                ),
            );
        }

        // Positive heuristics
        if subject_lower.starts_with("re:") || subject_lower.starts_with("fwd:") || subject_lower.starts_with("fw:") {
            ham_score += 1.0;
            matches.push(
                RuleMatch::new(
                    "threaded-subject",
                    "Assunto indica continuidade de conversa",
                    1.0,
                    RuleImpact::Ham,
                ),
            );
        }

        if contains_business_language(&body_lower) {
            ham_score += 1.1;
            matches.push(
                RuleMatch::new(
                    "business-language",
                    "Vocabulário típico de comunicação corporativa",
                    1.1,
                    RuleImpact::Ham,
                ),
            );
        }

        if body.contains("\n--\n") || body_lower.contains("atenciosamente") || body_lower.contains("best regards") {
            ham_score += 0.8;
            matches.push(
                RuleMatch::new(
                    "signature-present",
                    "Mensagem contém assinatura típica",
                    0.8,
                    RuleImpact::Ham,
                ),
            );
        }

        let (class, review) = self.finalize_class(spam_score, ham_score);

        ClassificationResult {
            class,
            spam_score,
            ham_score,
            rule_matches: matches,
            warnings,
            review_recommended: review,
        }
    }

    /// Classifies a fully structured email, analysing attachments and links.
    pub fn classify_email(&self, email: &Email) -> ClassificationResult {
        let mut result = self.classify_text(&email.subject, &email.body);
        self.inspect_attachments(&email.attachments, &mut result);
        self.inspect_links(email, &mut result);

        let (class, review) = self.finalize_class(result.spam_score, result.ham_score);
        result.class = class;
        result.review_recommended = review;
        result
    }

    /// Adds a custom spam rule using a regex pattern.
    pub fn add_spam_pattern(&mut self, pattern: Regex) {
        self.spam_rules.push(Rule::from_regex(
            "custom-spam",
            pattern,
            "Padrão de spam customizado",
            1.0,
            RuleImpact::Spam,
        ));
    }

    /// Adds a custom ham rule using a regex pattern.
    pub fn add_ham_pattern(&mut self, pattern: Regex) {
        self.ham_rules.push(Rule::from_regex(
            "custom-ham",
            pattern,
            "Padrão de ham customizado",
            1.0,
            RuleImpact::Ham,
        ));
    }

    fn finalize_class(&self, spam_score: f32, ham_score: f32) -> (EmailClass, bool) {
        let diff = spam_score - ham_score;
        let review = ((diff).abs() < self.review_margin && (spam_score >= self.min_review_score || ham_score >= self.min_review_score))
            || (spam_score >= self.spam_threshold - 0.5 && diff < self.spam_threshold);

        if diff >= self.spam_threshold {
            (EmailClass::Spam(spam_score.ceil() as usize), review)
        } else {
            (EmailClass::Ham(ham_score.ceil() as usize), review)
        }
    }

    fn inspect_attachments(&self, attachments: &[Attachment], result: &mut ClassificationResult) {
        for attachment in attachments {
            if let Some(ext) = file_extension(&attachment.filename) {
                if SUSPICIOUS_ATTACHMENT_EXTENSIONS.contains(&ext.as_str()) {
                    result.spam_score += 2.5;
                    result.rule_matches.push(
                        RuleMatch::new(
                            "suspicious-attachment",
                            "Anexo com extensão potencialmente maliciosa",
                            2.5,
                            RuleImpact::Spam,
                        )
                        .with_details(format!("{}", attachment.filename)),
                    );
                    result
                        .warnings
                        .push(format!("Revisar anexo suspeito: {}", attachment.filename));
                } else if TRUSTED_ATTACHMENT_EXTENSIONS.contains(&ext.as_str()) {
                    result.ham_score += 0.9;
                    result.rule_matches.push(
                        RuleMatch::new(
                            "trusted-attachment",
                            "Anexo com extensão corporativa recorrente",
                            0.9,
                            RuleImpact::Ham,
                        )
                        .with_details(format!("{}", attachment.filename)),
                    );
                }
            }
        }
    }

    fn inspect_links(&self, email: &Email, result: &mut ClassificationResult) {
        let mut link_count = 0;
        let mut suspicious_links = 0;

        for token in email
            .body
            .split_whitespace()
            .chain(email.html_body.iter().flat_map(|html| html.split_whitespace()))
        {
            let cleaned = token.trim_matches(|c: char| "\"'()[]{}<>,.;".contains(c));
            if cleaned.starts_with("http://") || cleaned.starts_with("https://") {
                link_count += 1;
                if is_suspicious_domain(cleaned) {
                    suspicious_links += 1;
                    result.spam_score += 1.7;
                    result.rule_matches.push(
                        RuleMatch::new(
                            "suspicious-link",
                            "Link apontando para domínio suspeito",
                            1.7,
                            RuleImpact::Spam,
                        )
                        .with_details(cleaned.chars().take(96).collect::<String>()),
                    );
                }
            }
        }

        if link_count >= 5 {
            result.spam_score += 1.2;
            result.rule_matches.push(
                RuleMatch::new(
                    "too-many-links",
                    "Mensagem contém muitos links externos",
                    1.2,
                    RuleImpact::Spam,
                )
                .with_details(format!("{} links detectados", link_count)),
            );
        }

        if suspicious_links > 0 {
            result
                .warnings
                .push(format!("{} link(s) suspeito(s) encontrado(s)", suspicious_links));
        }
    }

    fn default_spam_rules() -> Vec<Rule> {
        vec![
            Rule::new(
                "classic-scam",
                r"(?i)nigerian prince|inheritance|lottery winner|windfall",
                "Vocabulário clássico de golpes financeiros",
                1.5,
                RuleImpact::Spam,
            ),
            Rule::new(
                "pharmacy-offers",
                r"(?i)viagra|cialis|pharmacy|meds online",
                "Oferta de medicamentos controlados",
                1.4,
                RuleImpact::Spam,
            ),
            Rule::new(
                "high-pressure-sale",
                r"(?i)act now|last chance|limited time|urgent response",
                "Pressão para ação imediata",
                1.3,
                RuleImpact::Spam,
            ),
            Rule::new(
                "financial-bait",
                r"(?i)free money|get rich quick|double your income",
                "Promessa de ganhos financeiros irreais",
                1.4,
                RuleImpact::Spam,
            ),
            Rule::new(
                "crypto-scheme",
                r"(?i)crypto giveaway|token airdrop|defi opportunity",
                "Golpe envolvendo criptomoedas",
                1.2,
                RuleImpact::Spam,
            ),
            Rule::new(
                "lottery-claim",
                r"(?i)winner|jackpot|claim your prize",
                "Mensagem de falso prêmio",
                1.2,
                RuleImpact::Spam,
            ),
        ]
    }

    fn default_ham_rules() -> Vec<Rule> {
        vec![
            Rule::new(
                "meeting-context",
                r"(?i)meeting agenda|standup|follow up|minutes",
                "Termos comuns em emails corporativos",
                1.1,
                RuleImpact::Ham,
            ),
            Rule::new(
                "professional-language",
                r"(?i)please find attached|let's schedule|per our conversation",
                "Linguagem típica de comunicação profissional",
                1.0,
                RuleImpact::Ham,
            ),
            Rule::new(
                "gratitude-signoff",
                r"(?i)thank you|thanks|obrigado|agradeço",
                "Expressões de encerramento positivas",
                0.8,
                RuleImpact::Ham,
            ),
        ]
    }
}

impl Default for EmailClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmailClass {
    /// Email classificado como spam, contendo o score arredondado.
    Spam(usize),
    /// Email classificado como legítimo, contendo o score arredondado.
    Ham(usize),
}

/// Email threading - groups related emails
pub struct EmailThreader;

impl EmailThreader {
    /// Groups emails by conversation thread
    pub fn thread(emails: Vec<ThreadableEmail>) -> Vec<EmailThread> {
        let mut threads: Vec<EmailThread> = Vec::new();

        for email in emails {
            let mut found = false;

            // Try to add to existing thread
            for thread in &mut threads {
                if thread.belongs(&email) {
                    thread.add(email.clone());
                    found = true;
                    break;
                }
            }

            // Create new thread
            if !found {
                threads.push(EmailThread::new(email));
            }
        }

        threads
    }
}

#[derive(Debug, Clone)]
pub struct ThreadableEmail {
    pub id: String,
    pub subject: String,
    pub in_reply_to: Option<String>,
    pub references: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EmailThread {
    pub root_subject: String,
    pub emails: Vec<ThreadableEmail>,
}

impl EmailThread {
    fn new(email: ThreadableEmail) -> Self {
        let root_subject = Self::normalize_subject(&email.subject);
        Self {
            root_subject,
            emails: vec![email],
        }
    }

    fn belongs(&self, email: &ThreadableEmail) -> bool {
        let normalized = Self::normalize_subject(&email.subject);

        // Check subject match
        if normalized == self.root_subject {
            return true;
        }

        // Check references
        for existing in &self.emails {
            if email.in_reply_to.as_ref() == Some(&existing.id) {
                return true;
            }
            if email.references.contains(&existing.id) {
                return true;
            }
        }

        false
    }

    fn add(&mut self, email: ThreadableEmail) {
        self.emails.push(email);
    }

    fn normalize_subject(subject: &str) -> String {
        subject
            .trim()
            .to_lowercase()
            .replace("re:", "")
            .replace("fwd:", "")
            .replace("fw:", "")
            .trim()
            .to_string()
    }
}

fn uppercase_ratio(text: &str) -> (f32, usize) {
    let mut uppercase = 0usize;
    let mut letters = 0usize;
    for ch in text.chars() {
        if ch.is_alphabetic() {
            letters += 1;
            if ch.is_uppercase() {
                uppercase += 1;
            }
        }
    }

    if letters == 0 {
        (0.0, 0)
    } else {
        (uppercase as f32 / letters as f32, uppercase)
    }
}

fn contains_suspicious_base64(text: &str) -> bool {
    text.split_whitespace().any(|token| {
        token.len() >= 40
            && token.chars().all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '+' | '/' | '='))
    })
}

fn contains_business_language(body_lower: &str) -> bool {
    let indicators = [
        "meeting",
        "agenda",
        "call",
        "follow up",
        "project",
        "deliverable",
        "invoice",
        "proposal",
        "budget",
        "report",
    ];

    indicators.iter().any(|term| body_lower.contains(term))
}

fn file_extension(filename: &str) -> Option<String> {
    filename
        .rsplit('.')
        .next()
        .map(|ext| ext.trim().to_ascii_lowercase())
        .filter(|ext| !ext.is_empty())
}

fn is_suspicious_domain(url: &str) -> bool {
    let host_part = url
        .split("//")
        .nth(1)
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or(url)
        .trim_start_matches("www.")
        .trim();

    let domain = host_part.to_ascii_lowercase();

    if domain.is_empty() {
        return false;
    }

    const BAD_TLDS: &[&str] = &[".ru", ".cn", ".xyz", ".top", ".click", ".gq", ".ml", ".tk", ".work"];
    if BAD_TLDS.iter().any(|tld| domain.ends_with(tld)) {
        return true;
    }

    let digit_count = domain.chars().filter(|c| c.is_ascii_digit()).count();
    if digit_count >= 4 {
        return true;
    }

    domain.contains("--") || domain.contains('@')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{message::Attachment, EmailAddress};

    #[test]
    fn test_spam_classification() {
        let classifier = EmailClassifier::new();

        let result = classifier.classify(
            "URGENT!!! You've WON the LOTTERY",
            "Click HERE NOW to claim your FREE MONEY from the casino! Congratulations winner! Buy now viagra!",
        );

        assert!(matches!(result, EmailClass::Spam(_)));
    }

    #[test]
    fn test_ham_classification() {
        let classifier = EmailClassifier::new();

        let result = classifier.classify(
            "Team meeting tomorrow",
            "Hi everyone, let's meet at 2pm to discuss the project.",
        );

        assert!(matches!(result, EmailClass::Ham(_)));
    }

    #[test]
    fn test_classify_email_with_attachment() {
        let classifier = EmailClassifier::new();
        let from = EmailAddress::new("sender@corp.com").unwrap();
        let to = vec![EmailAddress::new("recipient@corp.com").unwrap()];
        let mut email = Email::new(from, to, "Invoice for Q4".into(), "Please find attached the invoice.".into());
        email.add_attachment(Attachment {
            filename: "invoice.pdf".into(),
            content_type: "application/pdf".into(),
            content: vec![],
        });

        let detailed = classifier.classify_email(&email);
        assert!(matches!(detailed.class, EmailClass::Ham(_)));
        assert!(detailed.rule_matches.iter().any(|m| m.name == "trusted-attachment"));
    }

    #[test]
    fn test_suspicious_attachment_flags_spam() {
        let classifier = EmailClassifier::new();
        let from = EmailAddress::new("alert@example.com").unwrap();
        let to = vec![EmailAddress::new("victim@example.com").unwrap()];
        let mut email = Email::new(from, to, "Security Update".into(), "Please install the attached update.".into());
        email.add_attachment(Attachment {
            filename: "update.exe".into(),
            content_type: "application/octet-stream".into(),
            content: vec![],
        });

        let detailed = classifier.classify_email(&email);
        assert!(detailed.is_spam());
        assert!(detailed.rule_matches.iter().any(|m| m.name == "suspicious-attachment"));
        assert!(detailed.warnings.iter().any(|w| w.contains("Revisar anexo")));
    }

    #[test]
    fn test_email_threading() {
        let emails = vec![
            ThreadableEmail {
                id: "1".to_string(),
                subject: "Hello".to_string(),
                in_reply_to: None,
                references: Vec::new(),
            },
            ThreadableEmail {
                id: "2".to_string(),
                subject: "Re: Hello".to_string(),
                in_reply_to: Some("1".to_string()),
                references: vec!["1".to_string()],
            },
        ];

        let threads = EmailThreader::thread(emails);
        assert_eq!(threads.len(), 1);
        assert_eq!(threads[0].emails.len(), 2);
    }
}
