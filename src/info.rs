// The Universal Permissive License (UPL), Version 1.0
// Copyright © 2023 Piotr Bajdek

// MODULE INFO

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

pub fn about(reset: &str, blue_underlined: &str, grey: &str, yellow: &str) {
    println!("{}", grey.to_owned() + "Program" + reset + ":  " + yellow + "mars-t" + reset);
    println!("{}", grey.to_owned() + "Version" + reset + ":  0.2.0");
    println!("{}", grey.to_owned() + "Date" + reset + ":     January 27, 2023");
    println!("{}", grey.to_owned() + "Author" + reset + ":   Piotr Bajdek");
    println!("{}", grey.to_owned() + "Contact" + reset + ":  " + blue_underlined + "piotr.bajdek@proton.me" + reset);
    println!("{}", grey.to_owned() + "Source" + reset + ":   " + blue_underlined + "https://github.com/piotrbajdek/mars-t" + reset);
    println!("{}", grey.to_owned() + "License" + reset + ":  UPL-1.0 Copyright © 2023 Piotr Bajdek");
}

pub fn help(reset: &str, grey: &str, violet: &str, yellow: &str) {
    println!("{}", grey.to_owned() + "Usage" + reset + ":     " + yellow + "mars-t [date] [time] [scale] [option]" + yellow);
    println!("           mars-t now [option]{}", reset.to_owned() + "  The current time");
    println!();
    println!("{}", grey.to_owned() + "Scales" + reset + ":    " + violet + "TAI" + reset + "      Temps Atomique International" + violet);
    println!("           TT{}", reset.to_owned() + "       Terrestrial Time" + violet);
    println!("           UTC{}", reset.to_owned() + "      Universal Coordinated Time");
    println!();
    println!("{}", grey.to_owned() + "Options" + reset + ":   " + violet + "MSD" + reset + "      Mars Sol Date" + violet);
    println!("           MTC{}", reset.to_owned() + "      Martian Coordinated Time" + violet);
    println!("           MSD+MTC{}", reset.to_owned() + "  MSD & MTC" + violet);
    println!("           MTC+MSD{}", reset.to_owned() + "  MTC & MSD");
    println!();
    println!("{}", grey.to_owned() + "Examples" + reset + ":  " + yellow + "mars-t 2023-03-20 12:30:00 UTC MSD");
    println!("           mars-t now MSD+MTC");
    println!();
    println!("{}", grey.to_owned() + "See also" + reset + ":  " + violet + "-a" + reset + ", " + violet + "--about" + reset + "       About this program");
    println!("           {}", violet.to_owned() + "-h" + reset + ", " + violet + "--help" + reset + "        This help");
    println!("           {}", violet.to_owned() + "-l" + reset + ", " + violet + "--license" + reset + "     The program license");
    println!("           {}", violet.to_owned() + "-v" + reset + ", " + violet + "--version" + reset + "     The program version");
}

pub fn version(reset: &str, grey: &str, yellow: &str) {
    println!("{}", grey.to_owned() + "Version" + reset + ": " + yellow + "0.2.0" + reset);
    println!("January 27, 2023");
}

pub fn license(reset: &str, yellow: &str) {
    println!("{}", yellow.to_owned() + "Copyright © 2023 Piotr Bajdek" + reset);
    println!();
    println!("The Universal Permissive License (UPL), Version 1.0");
    println!();
    println!("Subject to the condition set forth below, permission is hereby granted to any");
    println!("person obtaining a copy of this software, associated documentation and/or data");
    println!(r#"(collectively the "Software"), free of charge and under any and all copyright"#);
    println!("rights in the Software, and any and all patent rights owned or freely");
    println!("licensable by each licensor hereunder covering either (i) the unmodified");
    println!("Software as contributed to or provided by such licensor, or (ii) the Larger");
    println!("Works (as defined below), to deal in both");
    println!();
    println!("(a) the Software, and");
    println!();
    println!("(b) any piece of software and/or hardware listed in the lrgrwrks.txt file if");
    println!("one is included with the Software (each a “Larger Work” to which the Software");
    println!("is contributed by such licensors),");
    println!();
    println!("without restriction, including without limitation the rights to copy, create");
    println!("derivative works of, display, perform, and distribute the Software and make,");
    println!("use, sell, offer for sale, import, export, have made, and have sold the");
    println!("Software and the Larger Work(s), and to sublicense the foregoing rights on");
    println!("either these or other terms.");
    println!();
    println!("This license is subject to the following condition:");
    println!();
    println!("The above copyright notice and either this complete permission notice or at");
    println!("a minimum a reference to the UPL must be included in all copies or");
    println!("substantial portions of the Software.");
    println!();
    println!(r#"THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR"#);
    println!("IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,");
    println!("FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE");
    println!("AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER");
    println!("LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,");
    println!("OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE");
    println!("SOFTWARE.");
}
