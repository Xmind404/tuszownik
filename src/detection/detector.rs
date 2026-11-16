use regex::Regex;

pub struct Detection {
  pub category: String,
  pub value: String,
  pub start: usize,
  pub end: usize,
}


pub fn detect_sensitive_data(text_from_pdf: &str, patterns: &[(&str, &str)]) -> Vec<Detection> {
  let mut founded_values: Vec<Detection> = Vec::new();

  for (category, pattern) in patterns {
    let regex = Regex::new(pattern).unwrap();

    for found in regex.find_iter(text_from_pdf) {
      let start = found.start();
      let end = found.end();
      
      let already_detected = founded_values.iter().any(|detection| {
          detection.start == start && detection.end == end
      });

      if already_detected {
        continue;
      }

      founded_values.push(Detection { category: category.to_string(), value: found.as_str().to_string(), start, end });
    }
  }

  founded_values
}