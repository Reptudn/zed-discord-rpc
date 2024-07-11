use zed_extension_api as zed;

struct DiscordRPC {
    // ... state
}

impl zed::Extension for DiscordRPC {
    // ...
}

zed::register_extension!(DiscordRPC);
