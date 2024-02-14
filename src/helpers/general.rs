use crate::apis::call_request::call_gpt;
use crate::models::general::llm::Message;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;

// Extend ai function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) {
    let ai_function_str: &str = ai_func(func_input);

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {

        let x_str = convert_user_input_to_goal("dummy variable");

        dbg!(x_str);

    }

}