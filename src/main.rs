#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]

mod commands;
pub mod config;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use commands::{check_tablet, list_tablets, set_profile};

fn main() {
    // Handle the command line arguments
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("check")
            .about("Check if a specific tablet is connected. Prints message and returns status code")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::with_name("tablet")
                .help("Tablet config file name")
                .takes_value(true)
            )
        )
        .subcommand(
            SubCommand::with_name("list")
            .about("List all configured devices and their profiles")
        )
        .subcommand(
            SubCommand::with_name("profile")
            .about("Set a specific tablet to use a specific profile")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::with_name("tablet")
                .help("Tablet config file name")
                .takes_value(true)
            )
            .arg(
                Arg::with_name("profile")
                .help("Profile name")
                .takes_value(true)
            )
        )
        .get_matches();

    // Handle each subcommand
    match matches.subcommand() {
        ("check", Some(sub_matches)) => {
            let tablet_name = sub_matches.value_of("tablet").unwrap();
            let status = check_tablet(tablet_name);
            std::process::exit(status);
        }
        ("list", Some(_sub_matches)) => {
            list_tablets();
        }
        ("profile", Some(sub_matches)) => {
            let tablet_name = sub_matches.value_of("tablet").unwrap();
            let profile_name = sub_matches.value_of("profile").unwrap();
            set_profile(tablet_name, profile_name);
        }
        _ => {
            println!("{}", matches.usage());
            std::process::exit(1);
        }
    }
}
