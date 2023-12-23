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
      Explain me please code that I'll provide below.
      Explain in a way that I can understand it, and I'll be able to explain it to other people.
      Explain purpose of this file, code, blocks, functions, variables, and other things that you can find in this code.

      ###############################################
      File path that can be helpful for you: {}
      Code:
      {}

      Result:
      ",
      self.file,
      file_content.unwrap(),
    );
    let result = self.engine.ask(&prompt, None);

    result
  }
}