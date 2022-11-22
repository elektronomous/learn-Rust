fn main() {
    let _days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    // get the days length
    let n_days = _days.len();

    let _lyrics = [
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    // get the length of the lyrics
    let n_lyrics = _lyrics.len();

    for n in 0..n_days {
        println!("On the {} day of Christmas, my true love sent to me", _days[n]);
        for i in (0..n).rev() {
            if i == n_lyrics {
                break;
            }
            println!("{}", _lyrics[i])
        }
        println!("A partridge in a pear tree");
        println!("\n");
    }


    



}