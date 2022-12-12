use ansi_term::Colour::{Blue, Green, Purple, Red, Yellow};

fn main() {
    // 打印圣诞树
    println!("{}", Green.paint("     *     "));
    println!("{}", Green.paint("    ***    "));
    println!("{}", Green.paint("   *****   "));
    println!("{}", Green.paint("  *******  "));
    println!("{}", Green.paint(" ********* "));
    println!("{}", Green.paint("***********"));
    println!("{}", Yellow.paint("     ***     "));
    println!("{}", Yellow.paint("    *****    "));
    println!("{}", Purple.paint("   *******   "));
    println!("{}", Purple.paint("  *********  "));
    println!("{}", Blue.paint(" *********** "));
    println!("{}", Blue.paint("**************"));
    println!("{}", Red.paint("     ***     "));
    println!("{}", Red.paint("    *****    "));
    println!("{}", Red.paint("   *******   "));
    println!("{}", Red.paint("  *********  "));
    println!("{}", Red.paint(" *********** "));
    println!("{}", Red.paint("**************"));

    // 打印圣诞快乐
    println!("\n\n{}", Red.paint("Merry Christmas!"));
}
