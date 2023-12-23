use crate::{ai, env, fs};

use env::Env;
use openai_api_rust::{Message, Role};

pub struct RefactorRequest {
    file: String,
    engine: Box<dyn ai::AiQuestionable>,
}

impl RefactorRequest {
    pub fn new(file: String) -> RefactorRequest {
        let api_key = Env::get_api_key();

        RefactorRequest {
            file,
            engine: Box::new(ai::AI::new(api_key)),
        }
    }

    pub fn request_refactor(&self) -> String {
        let file_content = fs::FileSystem::read_file(&self.file);

        let context = vec![
            Message {
                role: Role::System,
                content: "
                  You're code refactor expert.
                  Your refactor improves code quality, and makes it more readable, and understandable.
                  Your refactor keeps best practices, and make code more maintainable.
              "
                .to_string(),
            },
            Message {
                role: Role::User,
                content: "
                  function helloworld(data) {
                    data = data || {};
                    data.message = data.message || 'Hello World!';
                    return data.message;
                  }
                  "
                .to_string(),
            },
            Message {
                role: Role::Assistant,
                content: "
        <find:>
        function helloworld(data) {
        <replace:>
        function helloworld(data = {}) {
        <message:>
        You can use default parameters to make code more readable.
        <find:>
          data = data || {};
        <replace:>
        <message:>
        You can use default parameters to make code more readable.
        <find:>
          data.message = data.message || 'Hello World!';
        <replace:>
        <message:>
          You're returning data.message, so you can use destructuring to make code more readable.
        "
                .to_string(),
            },
        ];
        let prompt: String = format!(
            r"
            If line needs refactor, Respond me this way, if no - just skip it.
            <find:>
            my code
            <replace:>
            your suggestion
            <message:>
            Explanation

      ###############################################
      File path that can be helpful for you: {}
      Code:
      {}

      Result:
      ",
            self.file,
            file_content.unwrap(),
        );
        let result = self.engine.ask(&prompt, Some(context));

        result
    }
}
