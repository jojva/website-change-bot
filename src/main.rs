use std::{
    hash::{DefaultHasher, Hash, Hasher},
    thread,
    time::Duration,
};

use lettre::{
    message::Message, transport::smtp::authentication::Credentials, SmtpTransport, Transport,
};

const RETRY_DELAY_SECS: u64 = 300;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut prev_h = 0;
    loop {
        let res = client.get("https://toto.com").send()?;
        let mut s = DefaultHasher::new();
        res.bytes()?.hash(&mut s);
        let h = s.finish();
        if prev_h != 0 && h != prev_h {
            send_email();
            break;
        }
        prev_h = h;
        thread::sleep(Duration::from_secs(RETRY_DELAY_SECS));
    }
    Ok(())
}

fn send_email() {
    let smtp_server = "some.server.com";
    let smtp_user = "user";
    let smtp_password = "pass";

    let email = Message::builder()
        .from("Ja Nette <ja.nette@janette.fr".parse().unwrap())
        .to("Jean Bon <jean.bon@perdu.com>".parse().unwrap())
        .subject("You won an iPad!")
        .body(String::from("Congratulations! You won an iPad! Please send us your credit card number to claim your prize."))
        .unwrap();

    let creds = Credentials::new(smtp_user.to_owned(), smtp_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay(smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
}
