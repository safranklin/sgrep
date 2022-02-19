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
    fn args_new_too_few_args() {
        assert!(Args::new(
            &vec!(
                String::from("./sgrep"),
                String::from("<query>")
            )
        ).is_err());
    }
    
    #[test]
    fn args_new_too_many_args() {
        assert!(Args::new(
            &vec!(
                String::from("./sgrep"),
                String::from("<query>"),
                String::from("<file_name>"),
                String::from("<extra_arg>")
            )
        ).is_err());
    }
    
    #[test]
    fn invalid_file() {
        let args: Args = Args::new(
            &vec!(
                String::from("./sgrep"),
                String::from("<query>"),
                String::from("./file_does_not_exist.error")
            )
        ).unwrap();
        
        assert!(run(args).unwrap_err().is::<std::io::Error>());
    }

}