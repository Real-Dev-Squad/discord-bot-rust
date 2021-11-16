mod channel_ids;

pub const HELP_COMMAND: &str = "?help";

pub fn get_help_message() -> String {
  return format!(
    "
  
  The bot is under development,
  
  If you are interested in building it help us out.
  
  To know more visit <{}>
  
  ",
    channel_ids::DISCORD_BOT_RUST
  );
}
