pub enum BanchoLobbyTeamModes {
    HeadToHead, // Values 0
    TagCoop,    // Values 1
    TeamVs,     // Values 2
    TagTeamVs,  // Values 3
}

pub enum BanchoLobbyTeams {
    Red,
    Blue,
}

pub enum BanchoLobbyPlayerStates {
    Ready,
    NotReady,
    NoMap,
}

pub enum ModeTypes {
    // TODO VERIFY THIS WHOLE LIST NAMES
    Osu,
    Taiko,
    Ctb,
    Mania,
}

pub enum BanchoLobbyWinConditions {
    Score,    // Values 0
    Accuracy, // Values 1
    Combo,    // Values 2
    ScoreV2,  // Values 3
}

pub enum ConnectStates {
    Disconnected,
    Connecting,
    Reconnecting,
    Connected,
}

pub enum BanchoMods {
    None,
    NoFail,
    Easy,
    Hidden,
    HardRock,
    SuddenDeath,
    DoubleTime,
    Relax,
    HalfTime,
    Nightcore,
    Flashlight,
    Autoplay,
    SpunOut,
    Relax2,
    Perfect,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    FadeIn,
    Random,
    LastMod,
    Key9,
    Key10,
    Key1,
    Key3,
    Key2,
}
