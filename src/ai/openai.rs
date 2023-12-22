use super::engine::AiQuestionable;
use openai_api_rust::{
    chat::{ChatApi, ChatBody},
    Auth, Message, OpenAI, Role,
};

// Implement Sized for AI

pub struct AI {
    engine: OpenAI,
}

impl AI {
    pub fn new(api_key: String) -> AI {
        let auth = Auth::new(&api_key);
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

        AI { engine: openai }
    }

    fn build_message(&self, message: &str) -> Vec<Message> {
      let mut messages = vec![];
      let system_message = Message {
          role: Role::System,
          content: "You're code explainer bot.
          You're explaining given code, what it's doing, for what code is responsible, purpose of code.".to_string(),
      };
      let user_message = Message {
        role: Role::User,
        content: message.to_string(),
      };
      messages.push(system_message);
      messages.push(user_message);

      return messages;
  }
}

impl AiQuestionable for AI {
    fn ask(&self, query: &str) -> String {
        let body = ChatBody {
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: Some(1000),
            temperature: None,
            top_p: None,
            n: None,
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            messages: self.build_message(query),
        };
        let rs = self.engine.chat_completion_create(&body);
        let choices = rs.unwrap().choices;
        let message = choices[0].message.as_ref().unwrap();

        return message.content.clone();
    }
}
