use crate::bancho_user::BanchoUser;

struct BanchoBotStats {
    user: BanchoUser,
    level: u32,
    accuracy: f32,
    status: String,
    online: bool,
}

/*

  TODO PARSE STATUS
const statusListener = (msg) => {
    const m = /Stats for \((.+)\)\[https:\/\/osu\.ppy\.sh\/u\/(\d+)\]( is (.+))?:/.exec(msg.message);
    if(m && this._usernameMatchesIrcUsername(m[1])) {
        this.user.username = m[1];
        this.user.id = Number(m[2]);
        this.return.status = m[4];
        this.return.online = m[4] != null;
    }
};
const scoreListener = (msg) => {
    const m = /Score: {4}(.+) \(#(\d+)\)/.exec(msg.message);
    if(m) {
        this.user.rankedScore = Number(m[1].replace(/,/g, ""));
        this.user.ppRank = Number(m[2]);
    }
};
const playsListener = (msg) => {
    const m = /Plays: {4}(\d+) \(lv(\d+)\)/.exec(msg.message);
    if(m) {
        this.user.playcount = Number(m[1]);
        // Levels returned by !stats are inaccurate so we're simply setting it in the return object instead.
        this.return.level = Number(m[2]);
    }
};
const accuracyListener = (msg) => {
    const m = /Accuracy: (\d+(\.\d+)?)%/.exec(msg.message);
    if(m) {
        // Accuracy returned by !stats are inaccurate so we're simply setting it in the return object instead.
        this.return.accuracy = Number(m[1]);
        this.resolve(this.return);
        this._removeListeners();
    }
};
 */

// TODO a function to send this this.user.banchojs.getUser("BanchoBot").sendMessage("!stats "+this.user.ircUsername);
