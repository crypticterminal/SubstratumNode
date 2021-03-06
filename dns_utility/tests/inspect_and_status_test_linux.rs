// Copyright (c) 2017-2018, Substratum LLC (https://substratum.net) and/or its affiliates. All rights reserved.
#![cfg (target_os = "linux")]
extern crate sub_lib;
extern crate dns_utility_lib;

mod utils;
mod linux_utils;

use dns_utility_lib::resolv_conf_dns_modifier::ResolvConfDnsModifier;
use utils::TestCommand;
use linux_utils::get_file_contents;
use linux_utils::get_nameserver_entries;
use linux_utils::is_subverted;

#[test]
// Any integration tests that should be run without root should have names ending in '_user_integration'
fn resolv_conf_inspect_and_status_user_integration () {
    let file_contents = match get_file_contents () {
        Ok (s) => s,
        Err (_) => {
            println! ("---INTEGRATION TEST CANNOT YET RUN IN THIS ENVIRONMENT---");
            return
        }
    };

    let mut inspect_command = TestCommand::start ("dns_utility", vec! ("inspect"));
    let exit_status = inspect_command.wait ();
    let output = inspect_command.output ();
    assert_eq! (exit_status, Some (0), "{}", output);
    assert_eq! (output, format! ("STANDARD OUTPUT:\n{}\nSTANDARD ERROR:\n\n", expected_inspect_output (&file_contents)));

    let mut status_command = TestCommand::start ("dns_utility", vec! ("status"));
    let exit_status = status_command.wait ();
    let output = status_command.output ();
    assert_eq! (exit_status, Some (0), "{}", output);
    assert_eq! (output, format! ("STANDARD OUTPUT:\n{}\nSTANDARD ERROR:\n\n", expected_status_output (&file_contents)));
}

fn expected_inspect_output (file_contents: &String) -> String {
    get_nameserver_entries (file_contents).into_iter ()
        .map (|nameserver_line| ResolvConfDnsModifier::new ().nameserver_line_to_ip(nameserver_line))
        .fold (String::new (), |so_far, entry| {
            format! ("{}{}\n", so_far, entry)
        })
}

fn expected_status_output (file_contents: &String) -> String {
    if is_subverted (&get_nameserver_entries (file_contents)) {"subverted\n"} else {"reverted\n"}.to_string ()
}