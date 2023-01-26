// The Universal Permissive License (UPL), Version 1.0
// Copyright © 2023 Piotr Bajdek

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

use std::env;
use std::process::exit;

pub mod info;

// MAIN

fn main() {
    let reset = "\x1b[0m";
    let blue_underlined = "\x1b[34;4m";
    let red = "\x1b[31m";
    let yellow = "\x1b[38;5;220m";
    let grey = "\x1b[38;5;240m";
    let violet = "\x1b[38;5;133m";

    for argument in env::args() {
        if argument == "-a" || argument == "--about" {
            info::about(reset, blue_underlined, grey, yellow);
            exit(0);
        } else if argument == "-h" || argument == "--help" {
            info::help(reset, grey, violet, yellow);
            exit(0);
        } else if argument == "-l" || argument == "--license" {
            info::license(reset, yellow);
            exit(0);
        } else if argument == "-v" || argument == "--version" {
            info::version(reset, grey, yellow);
            exit(0);
        }
    }

    let args: Vec<String> = env::args().collect();

    let arg1 = args.get(1).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));
    let arg2 = args.get(2).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));

    if arg1 == "now" {
        if arg2 == "MSD" {
            let mars_sol_date_now_float = mars_t::msd_now_flt();
            print!("{}", violet.to_owned() + "Current Mars Sol Date (MSD)" + reset + ": " + yellow);
            print!("{mars_sol_date_now_float}");
            println!("{reset}");
            return;
        } else if arg2 == "MTC" {
            let martian_coordinated_time_now_float = mars_t::mtc_now_flt();
            print!("{}", violet.to_owned() + "Current Martian Coordinated Time (MTC)" + reset + ": " + yellow);
            print!("{martian_coordinated_time_now_float}");
            println!("{reset}");

            let martian_coordinated_time_now_formatted = mars_t::mtc_now_fmt();
            print!("{}", violet.to_owned() + "Current Martian Coordinated Time (MTC)" + reset + ": " + yellow);
            print!("{martian_coordinated_time_now_formatted}");
            println!("{reset}");
            return;
        } else if arg2 == "MSD+MTC" {
            let mars_sol_date_now_float = mars_t::msd_now_flt();
            print!("{}", violet.to_owned() + "Current Mars Sol Date (MSD)" + reset + ": " + yellow);
            print!("{mars_sol_date_now_float}");
            println!("{reset}");

            let martian_coordinated_time_now_float = mars_t::mtc_now_flt();
            print!("{}", violet.to_owned() + "Current Martian Coordinated Time (MTC)" + reset + ": " + yellow);
            print!("{martian_coordinated_time_now_float}");
            println!("{reset}");

            let martian_coordinated_time_now_formatted = mars_t::mtc_now_fmt();
            print!("{}", violet.to_owned() + "Current Martian Coordinated Time (MTC)" + reset + ": " + yellow);
            print!("{martian_coordinated_time_now_formatted}");
            println!("{reset}");
            return;
        } else if arg2 == "MTC+MSD" {
            let martian_coordinated_time_now_float = mars_t::mtc_now_flt();
            print!("{}", violet.to_owned() + "Current Martian Coordinated Time (MTC)" + reset + ": " + yellow);
            print!("{martian_coordinated_time_now_float}");
            println!("{reset}");

            let martian_coordinated_time_now_formatted = mars_t::mtc_now_fmt();
            print!("{}", violet.to_owned() + "Current Martian Coordinated Time (MTC)" + reset + ": " + yellow);
            print!("{martian_coordinated_time_now_formatted}");
            println!("{reset}");

            let mars_sol_date_now_float = mars_t::msd_now_flt();
            print!("{}", violet.to_owned() + "Current Mars Sol Date (MSD)" + reset + ": " + yellow);
            print!("{mars_sol_date_now_float}");
            println!("{reset}");
            return;
        }
    }

    let date = arg1;
    let time = arg2;

    let scale = args.get(3).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));
    let arg4 = args.get(4).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));

    if arg4 == "MSD" {
        let mars_sol_date_float = mars_t::msd_flt(date, time, scale);
        print!("{}", violet.to_owned() + "Mars Sol Date (MSD)" + reset + ": " + yellow);
        print!("{mars_sol_date_float}");
        println!("{reset}");
        return;
    } else if arg4 == "MTC" {
        let martian_coordinated_time_float = mars_t::mtc_flt(date, time, scale);
        print!("{}", violet.to_owned() + "Martian Coordinated Time (MTC)" + reset + ": " + yellow);
        print!("{martian_coordinated_time_float}");
        println!("{reset}");

        let martian_coordinated_time_formatted = mars_t::mtc_fmt(date, time, scale);
        print!("{}", violet.to_owned() + "Martian Coordinated Time (MTC)" + reset + ": " + yellow);
        print!("{martian_coordinated_time_formatted}");
        println!("{reset}");
        return;
    } else if arg4 == "MSD+MTC" {
        let mars_sol_date_float = mars_t::msd_flt(date, time, scale);
        print!("{}", violet.to_owned() + "Mars Sol Date (MSD)" + reset + ": " + yellow);
        print!("{mars_sol_date_float}");
        println!("{reset}");

        let martian_coordinated_time_float = mars_t::mtc_flt(date, time, scale);
        print!("{}", violet.to_owned() + "Martian Coordinated Time (MTC)" + reset + ": " + yellow);
        print!("{martian_coordinated_time_float}");
        println!("{reset}");

        let martian_coordinated_time_formatted = mars_t::mtc_fmt(date, time, scale);
        print!("{}", violet.to_owned() + "Martian Coordinated Time (MTC)" + reset + ": " + yellow);
        print!("{martian_coordinated_time_formatted}");
        println!("{reset}");
        return;
    } else if arg4 == "MTC+MSD" {
        let martian_coordinated_time_float = mars_t::mtc_flt(date, time, scale);
        print!("{}", violet.to_owned() + "Martian Coordinated Time (MTC)" + reset + ": " + yellow);
        print!("{martian_coordinated_time_float}");
        println!("{reset}");

        let martian_coordinated_time_formatted = mars_t::mtc_fmt(date, time, scale);
        print!("{}", violet.to_owned() + "Martian Coordinated Time (MTC)" + reset + ": " + yellow);
        print!("{martian_coordinated_time_formatted}");
        println!("{reset}");

        let mars_sol_date_float = mars_t::msd_flt(date, time, scale);
        print!("{}", violet.to_owned() + "Mars Sol Date (MSD)" + reset + ": " + yellow);
        print!("{mars_sol_date_float}");
        println!("{reset}");
        return;
    }
    panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
}
