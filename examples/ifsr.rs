extern crate buerostatus;

use buerostatus::get_buerostatus;

fn main(){
    if let Some(status) = get_buerostatus(){
        if status { println!("Das Büro ist offen!"); }
        else { println!("Das Büro ist wahrscheinlich zu!"); }
    }
    else { println!("Kann gerade nicht nachschaun!"); }
}
