use crate::{fs, ai, env};

use env::Env;

pub struct Explainer {
  file: String,
  engine: Box<dyn ai::AiQuestionable>,
}

impl Explainer {
  pub fn new(file: String) -> Explainer {
    let api_key = Env::get_api_key();

    Explainer {
      file,
      engine: Box::new(ai::AI::new(api_key)),
    }
  }

  pub fn explain(&self) -> String {
    let file_content = fs::FileSystem::read_file(&self.file);


    // Multiline prompt
    let prompt = format!(
      r"
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