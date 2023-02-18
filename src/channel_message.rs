use crate::{bancho_channel::BanchoChannel, bancho_message::BanchoMessage};

// DONE
/**
 * Message received from a Channel
 */
struct ChannelMessage {
    channel: BanchoChannel,
    bancho_message: BanchoMessage,
}
