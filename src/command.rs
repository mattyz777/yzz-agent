pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "/copy",
        description: "Copy last message to clipboard",
    },
    Command {
        name: "/git",
        description: "Git commands",
    },
    Command {
        name: "/exit",
        description: "Exit application",
    },
];