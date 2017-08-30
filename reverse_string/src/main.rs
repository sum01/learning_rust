extern crate rprompt;

fn main() {
    loop {
        // Prompt for a reply on STDOUT
        let reply = rprompt::prompt_reply_stdout("Enter a string to reverse: ").unwrap();
        // Check if it's not empty
        if reply != "" {
            // Reverse the string
            let rev_reply = reply.chars().rev().collect::<String>();
            println!("\"{}\" reversed is \"{1}\"", reply, rev_reply);
        }
    }
}
