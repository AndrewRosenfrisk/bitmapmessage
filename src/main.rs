// Bitmap Message, by Al Sweigart al@inventwithpython.com
// Displays a text message according to the provided bitmap image.
// This code is available at https://nostarch.com/big-book-small-python-programming
// Tags: tiny, beginner, artistic

use std::io::stdin;

fn main() {
    let bitmap = "....................................................................
   **************   *  *** **  *      ******************************
  ********************* ** ** *  * ****************************** *
 **      *****************       ******************************
          *************          **  * **** ** ************** *
           *********            *******   **************** * *
            ********           ***************************  *
   *        * **** ***         *************** ******  ** *
               ****  *         ***************   *** ***  *
                 ******         *************    **   **  *
                 ********        *************    *  ** ***
                   ********         ********          * *** ****
                   *********         ******  *        **** ** * **
                   *********         ****** * *           *** *   *
                     ******          ***** **             *****   *
                     *****            **** *            ********
                    *****             ****              *********
                    ****              **                 *******   *
                    ***                                       *    *
                    **     *                    *
...................................................................."
        .to_string();

    println!("Bitmap Message, by Al Sweigart al@inventwithpython.com");
    println!("Enter the message to display with the bitmap.");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    print!("{esc}c", esc = 27 as char);

    input.remove(input.len() - 1);

    let new_bit_map = bitmap
        .split_inclusive("\n")
        .map(|line| {
            line.char_indices()
                .map(|c| {
                    if c.1 == ' ' || c.1 == '\n' {
                        c.1.to_string()
                    } else {
                        input.chars().nth(c.0 % input.len()).unwrap().to_string()
                    }
                })
                .collect::<String>()
        })
        .collect::<String>();

    println!("{new_bit_map}");
}
