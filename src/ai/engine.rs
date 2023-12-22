pub trait AiQuestionable {
    fn ask(&self, query: &str) -> String;
}