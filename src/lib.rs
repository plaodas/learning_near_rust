use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    password_solution: String
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(solution:String) -> Self {
        Self {
            password_solution:solution,
        }
    }

    pub fn get_solution(&self) -> String{
        self.password_solution.clone()
    }

    // pub fn set_solution(&mut self, solution: String){
    //     self.password_solution = solution;
    // }

    pub fn guess_solution(&mut self, solution: String) -> bool{
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        if hashed_input_hex == self.password_solution{
            env::log_str("You may enter! This is right password");
            true
        }else{
            env::log_str("You shall not pass. Try again.");
            false
        }

    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // debugging and iteration of a unit test
    // TESTS HERE
    #[test]
    fn debug_get_hash(){

        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "greens plaodas";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string)
    }

    #[test]
    fn check_guess_solution(){
        let plaodas = AccountId::new_unchecked("plaodas.testnet".to_string());
        let context = get_context(plaodas);
        testing_env!(context.build());

        let mut contract = Contract::new("d056cd17b39e01a3c3f0a4ea490d92eb19b80a559e6e74b59da4e84b2ec19209".to_string(),);

        let mut guess_result = contract.guess_solution("wrong_answer".to_string());
        assert!(!guess_result,"This is incorrect");
        assert_eq!(get_logs(), ["You shall not pass. Try again."], "Expected a failure in logs");


         guess_result = contract.guess_solution("greens plaodas".to_string());
        assert!(guess_result,"This is correct");
        assert_eq!(get_logs(), ["You shall not pass. Try again.","You may enter! This is right password"], "Expected a successful log after previous failed");


    }
    

}
