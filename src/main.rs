use clap::Parser;

mod cmd;
mod fs;
mod explainer;
mod ai;
mod env;
mod quality_analyzer;
mod refactor;

use explainer::Explainer;
use quality_analyzer::QualityAnalyzer;

use cmd::{Cmd, AnalyzeType};

fn main() {
  let cmd = Cmd::parse();

  let action = AnalyzeType::from_str(&cmd.action);
  
  match action {
    AnalyzeType::Explain => {
      print!("Explaining the code... \n");
      let explainer = Explainer::new(cmd.file);
      let result = explainer.explain();
      println!("{}", result);
    },
    AnalyzeType::AnalyzeQuality => {
      print!("Analyzing quality of the code... \n");
      let quality_analyzer = QualityAnalyzer::new(cmd.file);
      let result = quality_analyzer.qualify();

      println!("{}", result);
    },
    AnalyzeType::AnalyzeAll => {
      let explainer = Explainer::new(cmd.file.clone());
      let quality_analyzer = QualityAnalyzer::new(cmd.file);
      
      let explain_result = explainer.explain();
      let quality_result = quality_analyzer.qualify();


      println!("Explanation \n");
      println!("{}", explain_result);

      println!("\n\nAnalyze Result \n");
      println!("{}", quality_result);
    },
    AnalyzeType::Refactor => {
      println!("Refactoring the code... \n");
      let refactor = refactor::RefactorRequest::new(cmd.file);
      let result = refactor.request_refactor();

      println!("{}", result);
    }
  }
}
