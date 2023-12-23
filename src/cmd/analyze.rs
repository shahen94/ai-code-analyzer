pub enum AnalyzeType {
    Explain,
    AnalyzeQuality,
    AnalyzeAll,
    Refactor,
}

impl AnalyzeType {
    pub fn from_str(action: &str) -> AnalyzeType {
        match action {
            "explain" => AnalyzeType::Explain,
            "quality" => AnalyzeType::AnalyzeQuality,
            "analyze-all" => AnalyzeType::AnalyzeAll,
            "refactor" => AnalyzeType::Refactor,
            _ => AnalyzeType::Explain,
        }
    }
}