use crate::{fs, ai, env};

use env::Env;

pub struct QualityAnalyzer {
  file: String,
  engine: Box<dyn ai::AiQuestionable>,
}

impl QualityAnalyzer {
  pub fn new(file: String) -> QualityAnalyzer {
    let api_key = Env::get_api_key();

    QualityAnalyzer {
      file,
      engine: Box::new(ai::AI::new(api_key)),
    }
  }

  pub fn qualify(&self) -> String {
    let file_content = fs::FileSystem::read_file(&self.file);


    // Multiline prompt
    let prompt = format!(
      r"
      Analyze please this code, providing information about quality of this code, and how it can be improved.
      File path that can be helpful for you: {}
      Code:
      {}

      Result:
      ",
      self.file,
      file_content.unwrap(),
    );
    let result = self.engine.ask(&prompt);

    result
  }
}