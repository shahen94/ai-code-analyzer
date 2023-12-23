use openai_api_rust::Message;

pub trait AiQuestionable {
    fn ask(&self, query: &str, context: Option<Vec<Message>>) -> String;
}