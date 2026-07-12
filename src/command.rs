pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
}

pub const COMMANDS: &[Command] = &[
    Command { name: "/copy", description: "Copy to clipboard" },
    Command { name: "/clear", description: "Clear conversation" },
    Command { name: "/chat", description: "Switch session" },
    Command { name: "/config", description: "Open settings" },
    Command { name: "/editor", description: "Open multi-line editor" },
    Command { name: "/exit", description: "Exit session" },
    Command { name: "/help", description: "Show all commands" },
    Command { name: "/model", description: "Switch model" },
    Command { name: "/paste", description: "Paste image from clipboard" },
    Command { name: "/theme", description: "Change theme" },
];