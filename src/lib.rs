pub mod crossword_generator;
pub use crossword_generator::*;
// use tokio_stream::{Stream, StreamExt};
// use async_stream::stream;

// fn f() -> impl Stream
// {
//     stream!
//     {
//         async fn a(i: i32)
//         {
//             yield i;
//         }
//         for i in 0..33
//         {
//             a(5).await;
//         }
//     }
// }

// fn main() -> Result<(), Error>
// {
//     let generator: CrosswordGenerator = serde_json::from_reader(BufReader::new(File::open("input.json")?))?;

//     let mut output = File::create("output.txt")?;

//     let cws = generator.generate_crosswords();
//     for (ind, cw) in cws.iter().enumerate()
//     {
//         write!(output, "{}.\n{}\n\n\n", ind + 1, cw.generate_string())?;
//     }

//     println!("found {} crosswords", cws.len());

//     Ok(())
    
    
// }
