extern crate dialoguer;
#[cfg(feature = "validation")]
use dialoguer::validate::prebuilt::*;
use dialoguer::ValidatedInput;
fn main() {
    #[cfg(feature = "validation")]
    {
        let mut pinput: ValidatedInput<String, PhoneNumber> =
            ValidatedInput::new(PhoneNumber::default());
        let mut einput = ValidatedInput::new(EmailAddress::default());
        pinput.with_prompt("Enter a phone number");
        einput.with_prompt("Enter an email address");
        let ph = pinput.interact().unwrap();
        let e : String= einput.interact().unwrap();
        println!("{}\n{}", ph, e);
    }
}
