use crate::bancho_event::BanchoEvent;

// TODO events
/**
 * Bancho User
 */
pub struct BanchoUser {
    username_irc: String,
    user_id: u32,
    username: String,
    join_date: String, // TODO change to DateTime
    count300: u32,
    count100: u32,
    count50: u32,
    playcount: u32,
    ranked_score: u32,
    total_score: u32,
    pp_rank: u32,
    level: f32,
    pp_raw: f32,
    accuracy: f32,
    count_rank_ss: u32,
    count_rank_s: u32,
    count_rank_a: u32,
    country: String,
    total_seconds_played: u32,
    pp_country_rank: u32,
    events: Vec<BanchoEvent>,
}

impl BanchoUser {
    pub fn new() -> Self {
        Self {
            username_irc: String::new(),
            user_id: 0,
            username: String::new(),
            join_date: String::new(),
            count300: 0,
            count100: 0,
            count50: 0,
            playcount: 0,
            ranked_score: 0,
            total_score: 0,
            pp_rank: 0,
            level: 0.0,
            pp_raw: 0.0,
            accuracy: 0.0,
            count_rank_ss: 0,
            count_rank_s: 0,
            count_rank_a: 0,
            country: String::new(),
            total_seconds_played: 0,
            pp_country_rank: 0,
            events: Vec::new(),
        }
    }

    fn handle_event(&mut self, event: BanchoEvent) {
        // TODO handle events
    }
}
