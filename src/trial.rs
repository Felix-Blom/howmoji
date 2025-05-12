use inquire::{
    validator::{StringValidator, Validation},
    Text,
};

fn main() {

    let status = Text::new("What are you thinking about?")
        .with_validator(validator)
        .prompt();

    match status {
        Ok(status) => println!("Your status is being published..."),
        Err(err) => println!("Error while publishing your status: {}", err),
    }
}
