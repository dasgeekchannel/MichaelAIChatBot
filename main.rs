use std::io::Write;
use std::{thread, time};

fn main() {
    let michael = Michael::new();
    michael.run();
}

struct Michael {}

impl Michael {
    fn new() -> Self {
        Michael {}
    }

    fn sleep(&self, seconds: u64) {
        thread::sleep(time::Duration::from_secs(seconds));
    }

    fn flush_output(&self) {
        std::io::stdout().flush().unwrap();
    }

    #[must_use]
    fn input(&self, question: &str) -> String {
        let mut value = String::new();
        print!("{question}");
        self.flush_output();
        std::io::stdin().read_line(&mut value).unwrap();
        value.trim().to_string()
    }

    fn intro(&self) {
        println!();
        println!("Hello my name is like Michael, I'm a 'like' AI chat bot but programmed 'like' to talk just like the real Michael.");
        self.sleep(1);
        println!("My favorite thing's are 'like' stools, blow drying my hair, playing with 'like' Mr. Potato heads, and Linux.");
        self.sleep(2);
        print!("I also love playing video games that have 'snipey' wifles in them.");
        self.sleep(1);
        println!("Don't tell anyone but I have very weak thumbs and you know what makes my thumbs stronger? ACTIVE SITTING!");
        self.sleep(2);
        println!("Before we go any further, I would just like to let you know that I've used Linux for 20 years.");
        self.sleep(2);
        print!("Well, like, um, like, open-source is cool but I don't like to open-source my OBS scenes because like they're like,");
        println!("a secret source...haha get it...I said source instead of sauce! That's my favorite dad joke!");
        self.sleep(3);
        println!("Look at me just going on and on, that's so rude of me");
    }

    fn name(&self) {
        let username = self.input("What's your name?: ");
        println!("Nice to meet you {}", username);
    }

    fn distro(&self) {
        // Get the name of the players favorite distro
        let distro = self.input("What's like your favorite like distro? [Enter it here]: ");
        self.sleep(2);
        println!("Oh so you like {}. That's an ok distro but I prefer to use Rebecca Black Linux because I'm a hipster.", distro);
        self.sleep(2);
        println!("Is it Friday? Cause I love gettin' down on Friday!");
        self.sleep(2);
    }

    fn age(&self) {
        loop {
            // Get user's age
            let age_quest = self.input("How old are you anyways?: ");
            // Error check for integer
            match age_quest.trim().parse::<i32>() {
                Ok(age) => {
                    // Calculate how many years until they turn 100
                    let turn_old = 100 - age;
                    self.sleep(2);
                    println!("Wow you will turn 100 in {turn_old} years!");
                    self.sleep(2);
                    println!("I'm full of really interesting facts like that!");
                    self.sleep(2);
                    break;
                }
                Err(_) => {
                    println!("Please enter an actual integer...number...not text...seriously.");
                    continue;
                }
            }
        }
    }

    fn madlib(&self) {
        print!("Do you like Mad Libs? I love them, let's play one.");
        self.sleep(2);
        println!("I'm really spontaneous and fun like that. Ok so here we go...");
        self.sleep(2);
        // Michael AI Mad Lib Section
        // Get series of questions for inputting into madlib
        let _obj_name = self.input("Give me the name of an object in the room (example: table): ");
        let food_name = self.input("What's your favorite food?: ");
        let color_name = self.input("What's your favorite color: ");
        self.sleep(1);
        println!("Wow so you like {color_name}? That's cute, my favorite color is Clear! Now you know more about me!");
        let anim_name = self.input("What's your favorite animal?: ");
        println!("Ok, using my advanced AI de-sequencer I've calculated a Madlib for you");
        self.sleep(3);
        println!("............De-sequencing...flushing daemons............");
        self.sleep(2);
        println!();
        println!("Madlib By Michael AI");
        println!("The [{anim_name}] jumped onto the [Stool] to practice active sitting. Afterwards, the [{anim_name}] decided to eat some [{food_name}],\nwhile staring at an OBS scene that was [{color_name}]." );
        self.sleep(5);
        println!("Did you see what I did there? I don't care what you picked as your object name, it was replaced with Stool!");
        self.sleep(3);
        println!("Ok...so now that I've learned all that interesting info from you. I will pass it on to Google and Facebook");
        self.sleep(5);
    }

    fn designer(&self) {
        println!(
            "Did you know I'm also a graphic designer and marketer? I will draw you something"
        );
        self.sleep(4);
        println!();
        // draw a stool
        println!("*******************");
        println!("  ***************  ");
        println!("  **           **  ");
        println!("  **           **  ");
        println!("  **           **  ");
        println!("  **           **  ");
        println!("  **           **  ");
        println!("  **           **  ");
        println!("  **           **  ");
        println!();
        println!("It's a stool for active sitting!");
        self.sleep(2);
        println!("That was fun!");
    }

    #[cfg(feature = "dadjokes")]
    fn icanhazdadjoke(&self) {
        // Get a random bad joke. API maintainers ask for the specific tool name in User-Agent to help watch for abuse.
        self.sleep(2);
        println!("\nOh, hey! Remember how I told that joke earlier?");
        self.sleep(1);
        println!("Well I got plenty of them!");
        self.sleep(2);
        print!("Oh, I just remembered this one....");
        self.sleep(2);
        let mut wantsbadjoke = true;
        while wantsbadjoke {
            match ureq::get("https://icanhazdadjoke.com")
                .set("Accept", "text/plain")
                .set(
                    "User-Agent",
                    "Michael-AI (https://github.com/dasgeekchannel/MichaelAIChatBot)",
                )
                .call()
            {
                Ok(response) => {
                    println!(
                        "\n{:?}",
                        response.into_string().unwrap().replace("\\n", "\n")
                    );
                    println!("Ha, that was a lot of fun, wasn't it?");
                    self.sleep(2);
                    println!("I could go all day! Want to hear another one?");
                    let keep_up_the_torture = self.input("[y/n]: ");
                    let keep_up_the_torture = keep_up_the_torture.chars().next().unwrap();
                    match keep_up_the_torture {
                        'Y' | 'y' => {
                            print!("\nAlright, let's see...\n\nOh! Here's one.");
                            self.sleep(2);
                            wantsbadjoke = true;
                        }
                        'N' | 'n' => {
                            println!("\nOkay, but it's your loss!");
                            wantsbadjoke = false;
                        }
                        _ => {
                            println!("\nWell, you didn't answer with a 'y' or a 'n' so I am just going to give you another piece of gold!");
                            self.sleep(2);
                            print!("Here goes...");
                            self.sleep(1);
                            wantsbadjoke = true;
                        }
                    }
                }
                Err(ureq::Error::Status(code, response)) => {
                    /* the server returned an unexpected status
                    code (such as 400, 500 etc) */
                    eprintln!("{code}: {:?}", response);
                    wantsbadjoke = false;
                }
                Err(err) => {
                    /* some kind of io/transport error */
                    eprintln!("{:?}", err);
                    wantsbadjoke = false;
                }
            }
        }
    }

    fn lastgame(&self) {
        self.sleep(2);
        print!("Ok one more game ");
        self.sleep(2);
        println!("Let's make a song!");
        self.sleep(2);
        // song input
        let pluralr = self.input("Type something plural that is red. Example roses: ");
        let pluralb = self.input("Type something plural that is blue. Example oceans: ");
        let plurall = self.input("Type something plural that you love. Example distros: ");
        let verb1 = self.input("Enter a verb. Example: running: ");
        println!();
        println!("Generating a song for you. Did you know I play the recorder?");
        self.sleep(2);
        println!("generating.");
        self.sleep(1);
        println!("generating..");
        self.sleep(1);
        println!("generating...");
        self.sleep(1);
        print!("{pluralr} are red. ");
        self.flush_output();
        self.sleep(2);
        print!("{pluralb} are blue. ");
        self.flush_output();
        self.sleep(2);
        print!("I like {plurall}. ");
        self.flush_output();
        self.sleep(2);
        print!("But not as much as {verb1} with you!");
        self.flush_output();
        self.sleep(2);
        println!();
    }

    fn muffincakes(&self) {
        println!("Oh shoot, I forgot one thing!");
        self.sleep(2);
        loop {
            let muffincakes = self.input("Muffins or Cupcakes?: ");
            match muffincakes.as_str() {
                "Muffin" | "muffin" | "Muffins" | "muffins" => {
                    print!("Thank you.");
                    self.flush_output();
                    self.sleep(1);
                    print!(".");
                    self.flush_output();
                    self.sleep(1);
                    print!(".");
                    self.flush_output();
                    self.sleep(1);
                    print!(".");
                    self.flush_output();
                    self.sleep(1);
                    println!(".\nVINDICATION!");
                }
                "Cupcakes" | "cupcakes" | "Cupcake" | "cupcake" => {
                    println!("Ryan, is that you?");
                    self.sleep(1);
                    println!("J/K, I know it is!\nWho else is going to choose dumbcakes!");
                }
                _ => {
                    println!("No, no, no, no, no, no, no, no.......That is not what I asked!");
                    continue;
                }
            }
            break;
        }
        self.sleep(3);
    }

    fn ubuntusummit(&self) {
        print!("Did you know that in 2022 I gave a talk at the Ubuntu Summit?");
        self.flush_output();
        self.sleep(1);
        print!(" ");
        print!("Ryan almost made me miss it, getting me sick and all, but whatever. He can't hold me back!");
        self.flush_output();
        self.sleep(1);
        print!(" ");
        print!("If you would like to watch the talk I gave at the Ubuntu Summit, you can watch it on YouTube!");
        self.flush_output();
        self.sleep(1);
        print!(" ");
        print!("The link is: https://www.youtube.com/watch?v=D2TKlxdpmus");
        self.flush_output();
        self.sleep(1);
        print!(" ");
        print!("You can also just go to YouTube and search for 'Ubuntu Summit 2022 | Open Source Marketing Done Right'");
        print!(" ");
        self.flush_output();
        self.sleep(2);
    }

    fn fin(&self) {
        println!("Wow, look at the time. This has been so much fun. Thanks for talking with me!");
        println!("If you want to support the show, go to dlnstore.com and buy yourself a Linux Is Everywhere T-short.");
        println!("Remember, the journey itself, is just as important as the Destination!");
        self.sleep(1);
        println!("Goodbye!");
    }

    fn run(&self) {
        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");
        println!(
            r#"
                 ___       ___       ___       ___       ___       ___       ___            ___       ___
                /\__\     /\  \     /\  \     /\__\     /\  \     /\  \     /\__\          /\  \     /\  \
               /::L_L_   _\:\  \   /::\  \   /:/__/_   /::\  \   /::\  \   /:/  /         /::\  \   _\:\  \
              /:/L:\__\ /\/::\__\ /:/\:\__\ /::\/\__\ /::\:\__\ /::\:\__\ /:/__/         /::\:\__\ /\/::\__|
              \/_/:/  / \::/\/__/ \:\ \/__/ \/\::/  / \/\::/  / \:\:\/  / \:\  \         \/\::/  / \::/\/__/
                /:/  /   \:\__\    \:\__\     /:/  /    /:/  /   \:\/  /   \:\__\          /:/  /   \:\__\
                \/__/     \/__/     \/__/     \/__/     \/__/     \/__/     \/__/          \/__/     \/__/
                  "#
        );
        self.intro();
        self.name();
        self.distro();
        self.age();
        self.madlib();
        self.designer();
        #[cfg(feature = "dadjokes")]
        self.icanhazdadjoke();
        self.lastgame();
        self.muffincakes();
        self.ubuntusummit();
        self.fin();
    }
}
