use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"] 
pub struct Grammar; 

fn main() -> anyhow::Result<()> {
    let input = "-123.5,-15\n";  
    let gor = Grammar::parse(Rule::file, input)?;
    println!("{:?}", gor); 

    Ok(())
    }
