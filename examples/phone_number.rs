extern crate dialoguer;
#[cfg(feature = "validation")]
use dialoguer::validate::prebuilt::PhoneNumber;
use dialoguer::ValidatedInput;
fn main() {
    #[cfg(feature = "validation")]
    {
        let mut pinput: ValidatedInput<String, PhoneNumber> =
            ValidatedInput::new(PhoneNumber::default());
        pinput.input.with_prompt("Enter a phone number");
        let ph = pinput.interact().unwrap();
        println!("{}", ph);
    }
}
