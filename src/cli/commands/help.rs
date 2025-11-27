use crate::error::TorrerResult;

/// Show detailed help for a command
pub fn show_help(command: Option<&str>) -> TorrerResult<()> {
    if let Some(cmd) = command {
        show_command_help(cmd)
    } else {
        show_general_help()
    }
}

fn show_general_help() -> TorrerResult<()> {
    println!("Torrer - System-wide Tor routing for Ubuntu");
    println!();
    println!("USAGE:");
    println!("    torrer [SUBCOMMAND]");
    println!();
    println!("SUBCOMMANDS:");
    println!("    start              Start Tor routing");
    println!("    stop               Stop Tor routing");
    println!("    status             Show routing status");
    println!("    restart            Restart Tor routing");
    println!("    config             Interactive configuration");
    println!("    add-bridge         Add a bridge");
    println!("    list-bridges       List configured bridges");
    println!("    test-bridge        Test bridge connectivity");
    println!("    logs               View logs");
    println!("    export             Export configuration");
    println!("    import             Import configuration");
    println!("    stats              Show statistics");
    println!("    set-country        Set exit node country");
    println!("    randomize-mac       Randomize MAC addresses");
    println!("    validate           Validate installation");
    println!("    health             Health check");
    println!("    test               Run diagnostic tests");
    println!("    diagnostics        Collect diagnostics");
    println!("    completion         Generate shell completions");
    println!("    clean              Clean temporary files");
    println!("    info               Show system information");
    println!("    leak-test          Test for DNS/IPv6 leaks");
    println!("    circuits           List Tor circuits");
    println!("    new-circuit        Request new Tor circuit");
    println!("    state              Show application state");
    println!("    save-state         Save state to file");
    println!("    load-state         Load state from file");
    println!("    check-update        Check for updates");
    println!("    update             Update Torrer");
    println!("    backup             Create backup");
    println!("    list-backups       List backups");
    println!("    restore-backup     Restore from backup");
    println!("    clean-backups      Clean old backups");
    println!();
    println!("For more information about a specific command, run:");
    println!("    torrer help <command>");
    println!();
    println!("For more information, see: https://github.com/yourusername/torrer");

    Ok(())
}

fn show_command_help(command: &str) -> TorrerResult<()> {
    match command {
        "start" => {
            println!("Start Tor routing");
            println!();
            println!("This command will:");
            println!("  - Connect to Tor daemon");
            println!("  - Configure iptables for transparent routing");
            println!("  - Set up DNS leak prevention");
            println!("  - Disable IPv6 (prevent leaks)");
            println!();
            println!("Requires: sudo");
        }
        "stop" => {
            println!("Stop Tor routing");
            println!();
            println!("This command will:");
            println!("  - Remove Tor routing rules");
            println!("  - Restore original iptables configuration");
            println!("  - Restore DNS settings");
            println!();
            println!("Requires: sudo");
        }
        "status" => {
            println!("Show routing status");
            println!();
            println!("Displays:");
            println!("  - Routing status (active/inactive)");
            println!("  - Tor connection status");
            println!("  - Circuit establishment status");
        }
        _ => {
            println!("Help for '{}' command", command);
            println!("(Detailed help not yet available)");
        }
    }

    Ok(())
}

