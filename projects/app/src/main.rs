// use lib::hello;

mod letter_order_print;
use letter_order_print::lwrcs_to_uprcs;
use letter_order_print::special_situation::uprcs_to_lwrcs;

fn main() {
    // hello();
    lwrcs_to_uprcs::print_la_to_uz();

    println!(" ");

    uprcs_to_lwrcs::print_ua_to_lz();
}
