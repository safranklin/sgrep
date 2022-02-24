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
    pub fn new<'a, T: ExactSizeIterator<Item = String>>(args: T) -> Result<Args, &'a str> {
        // Here we have a generic argument of type T which implements the ExactSizeIterator
        // trait which has Items of type String. The reason we are using a generic argument here
        // is so we can write tests where we build up a vector of "Arguments".
        
        // We use ExactSizeIterator so that we can call .len() here:
        if args.len() > 3 {
            return Err("Incorrect number of arguments!");
        }

        // Check to see that the correct number of arguments exists (need 3)
        let mut args = args.skip(1);

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing 'query' argument.")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing 'file_path' argument.")
        };

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

    // Let's use some iterator adaptors
    contents
    .lines() // For each line from the contents
    .filter(|line| line.contains(query)) // Filter it to only lines which contain the query string
    .collect() // Return the collection of the filter iterator
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Searches for query through the lines in the contents.
    // Returns the line that contain query.

    // Use lifetime annotation: 'a 
    // to tell Rust the data returned by search will only live as long as the data that was passed
    // in via contents.

    // Shadow previous query value with a reference to query lowercased.
    let query = &query.to_lowercase();

    contents
    .lines() // For each line from the contents
    .filter(
        |line| 
        line.to_lowercase().contains(query) // Filter for lines that (when lowercased) 
        // contains the content of query (also lowercased)
    ) // Filter it to only lines which contain the query string
    .collect() // Return the collection of the filter iterator
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
            vec!(
                String::from("./sgrep"),
                String::from("<query>")
            )
            .iter()
            .map(|s| s.to_string())
        ).is_err());
    }
    
    #[test]
    fn args_new_too_many_args() {
        assert!(Args::new(
            vec!(
                String::from("./sgrep"),
                String::from("<query>"),
                String::from("<file_name>"),
                String::from("<extra_arg>")
            )
            .iter()
            .map(|s| s.to_string())
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
            vec!(
                String::from("./sgrep"),
                String::from("<query>"),
                String::from("./file_does_not_exist.error")
            )
            .iter()
            .map(|s| s.to_string())
        ).unwrap();
        
        assert!(run(args).unwrap_err().is::<std::io::Error>());
    }
    // End:     === run tests ===

}