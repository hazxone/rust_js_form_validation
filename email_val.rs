use regex::Regex;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

fn is_valid_email(email: &str) -> bool {
    // Step 1
    if !email.contains('@') {
        return false;
    }

    // Step 2
    let mut parts = email.split('@');
    let username = parts.next().unwrap();
    let domain = parts.next().unwrap();

    // Step 3
    let username_regex = Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap();
    let domain_regex = Regex::new(r"^[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    if !username_regex.is_match(username) || !domain_regex.is_match(domain) {
        return false;
    }

    // Step 4
    if !domain.contains('.') {
        return false;
    }

    // Step 5
    if domain.ends_with('-') || domain.ends_with('.') {
        return false;
    }

    // Step 6
    if email.len() > 254 {
        return false;
    }

    // Step 7 (optional)
    let config = ResolverConfig::default();
    let opts = ResolverOpts::default();
    let resolver = Resolver::new(config, opts).unwrap();
    match resolver.mx_lookup(domain) {
        Ok(_) => true,
        Err(_) => false,
    }
}
