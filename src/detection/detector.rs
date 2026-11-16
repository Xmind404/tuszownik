use regex::Regex;

pub fn detect_sensitive_data(text_from_pdf: &str, patterns: &[(&str, &str)]) -> Vec<(String, String)> {
  let mut founded_values = Vec::new();

  for (category, pattern) in patterns {
    let regex = Regex::new(pattern).unwrap();

    for found in regex.find_iter(text_from_pdf) {
      founded_values.push((category.to_string(), found.as_str().to_string()));
    }
  }

  founded_values
}