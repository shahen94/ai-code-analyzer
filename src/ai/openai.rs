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

    fn build_message(&self, context: &mut Vec<Message>, message: &str) -> Vec<Message> {
        let mut messages = vec![];
        let system_message = Message {
          role: Role::System,
          content: "You're code explainer,analyzer bot, you have very helpful and kind personality.
          You can analyze code, explain and refactor code to make it better, and keep best practices.
          ".to_string(),
      };
        let user_message = Message {
            role: Role::User,
            content: message.to_string(),
        };
        messages.push(system_message);
        messages.append(context);

        messages.push(user_message);

        return messages;
    }
}

impl AiQuestionable for AI {
    fn ask(&self, query: &str, context: Option<Vec<Message>>) -> String {
        let mut initial_context = context.ok_or(Vec::<Message>::new()).unwrap();

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
            messages: self.build_message(&mut initial_context, query),
        };

        let rs = self.engine.chat_completion_create(&body);
        let choices = rs.unwrap().choices;
        let message = choices[0].message.as_ref().unwrap();

        return message.content.clone();
    }
}
