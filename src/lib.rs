use std::error::Error;
use std::fs;
use std::env;

#[derive(Debug, Clone)]
pub struct Args {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool
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

        // Read the CASE_INSENSITIVE value off the environment
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Args {
            query,
            file_path,
            case_sensitive
        })
    }
    
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Searches for query through the lines in the contents.
    // Returns the line that contain query.

    // Use lifetime annotation: 'a 
    // to tell Rust the data returned by search will only live as long as the data that was passed
    // in via contents.

    let mut matches = vec![];
    
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Searches for query through the lines in the contents.
    // Returns the line that contain query.

    // Use lifetime annotation: 'a 
    // to tell Rust the data returned by search will only live as long as the data that was passed
    // in via contents.

    // Shadow previous query value with a reference to query lowercased.
    let query = &query.to_lowercase();

    let mut matches = vec![];
    
    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            matches.push(line);
        }
    }

    matches
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // Read the string, if an error occurs return it to the caller for handling using the '?' operator.
    let contents: String = fs::read_to_string(&args.file_path)?;

    let results: Vec<&str>;
    
    if args.case_sensitive {
        results = search(&args.query, &contents);
    } else {
        results = search_case_insensitive(&args.query, &contents);
    }

    for line in results {
        println!("{}", line);
    }
    
    // Upon success return the unit type '()'
    Ok(())
}


#[cfg(test)]
mod tests {

    use super::*;
    
    // Start:   === Args::new tests ===
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

    // End:     === Args::new tests ===
    
    // Start:   === search tests ===
    #[test]
    fn case_one_match() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_multi_match() {
        let query = "t";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:", "safe, fast, productive.", "Pick three."], search(query, contents));
    }

    #[test]
    fn no_matches() {
        let query = "test";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Vec::<String>::new(), search(query, contents));
    }

    #[test]
    fn case_insensitive_one_match() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    // End:     === run tests ===
    
    // Start:   === run tests ===
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
    // End:     === run tests ===

}