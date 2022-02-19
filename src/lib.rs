use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
pub struct Args {
    pub query: String,
    pub file_path: String,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &str> {
        // Check to see that the correct number of arguments exists (need 3)
        if args.len() != 3 {
            return Err("Wrong number of arguments.")
        }

        // We are going to use .clone() here despite the fact that it will have a performance impact.
        // The tradeoffs of simplicity in this case for a little bit of performance is acceptable,
        // especially considering the size of the two arguments is going to be low.
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Args { query, file_path })
    }
    
}


pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // Read the string, if an error occurs return it to the caller for handling using the '?' operator.
    let contents: String = fs::read_to_string(&args.file_path)?;
    println!("File {} contains contents:\n{}", args.file_path, contents);
    
    
    // Upon success return the unit type '()'
    Ok(())
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn args_new_too_few_args() {
        match Args::new(
            &vec!(
                String::from("./sgrep"),
                String::from("<query>")
            )
        ) {
            Ok(val) => println!("Successfully initialized! {} {}", val.query, val.file_path),
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    #[should_panic]
    fn args_new_too_many_args() {
        match Args::new(
            &vec!(
                String::from("./sgrep"),
                String::from("<query>"),
                String::from("<file_path>"),
                String::from("<extra_arg>")
            )
        ) {
            Ok(val) => println!("Successfully initialized! {} {}", val.query, val.file_path),
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn invalid_file() {
        if let Ok(args) = Args::new(
            &vec!(
                String::from("./sgrep"),
                String::from("<query>"),
                String::from("./some_path_that_does_not_exist")
            )
        ) {
            if let Err(e) = run(args) {
                assert!(e.is::<std::io::Error>());
            } else {
                panic!("File path should not be valid.");
            }
        }
        else {
            panic!("Couldn't instantiate valid constructor.");
        }
    }

}