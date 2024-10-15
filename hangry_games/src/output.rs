// Collection on strings to be used as output for the game

use std::fmt::format;

// Game messages
pub const GAME_DAY_START: &str = "=== ☀️ Day {} begins! ===";
pub const FIRST_DAY_START: &str = "=== 🎉 The Hunger Games begin! 🎉 ===";
pub const FEAST_DAY_START: &str = "=== 😋 Day 3: Feast Day ===";
pub const TRIBUTES_LEFT: &str = "=== 📌 Tributes alive: {} ===";
pub const GAME_NIGHT_START: &str = "=== 🌙 Night {} begins ===";
pub const DAILY_DEATH_ANNOUNCEMENT: &str = "=== 💀 Tributes dead: {} ===";
pub const DEATH_ANNOUNCEMENT: &str = "=== 🪦 {} has died ===";

// End game messages
pub const NO_ONE_WINS: &str = "=== 🎭 No one wins! ===";
pub const TRIBUTE_WINS: &str = "=== 🏆 The winner is {} ===";

// Tribute messages
pub const TRIBUTE_REST: &str = "😪 {} rests";
pub const TRIBUTE_LONG_REST: &str = "💤 {} rests and recovers a little health and sanity";
pub const TRIBUTE_HIDE: &str = "🫥 {} tries to hide";
pub const TRIBUTE_TRAVEL: &str = "🚶 {} moves from {} to {}";
pub const TRIBUTE_TAKE_ITEM: &str = "🔨 {} takes a(n) {}";
pub const TRIBUTE_CANNOT_USE_ITEM: &str = "❌ {} cannot use a(n) {}";
pub const TRIBUTE_USE_ITEM: &str = "💊 {} uses a(n) {}, gains {} {}";
pub const TRIBUTE_TRAVEL_TOO_TIRED: &str = "😴 {} is too tired to move from {}, rests instead";
pub const TRIBUTE_TRAVEL_ALREADY_THERE: &str = "🤔 {} is already in the suggested area, stays put";
pub const TRIBUTE_TRAVEL_FOLLOW: &str = "🫡 {} follows their district mate to {}";
pub const TRIBUTE_TRAVEL_STAY: &str = "🪑 {} stays in {}";
pub const TRIBUTE_BLEEDS: &str = "🩸 {} bleeds from their wounds.";
pub const TRIBUTE_SICK: &str = "🤒 {} contracts dysentery, loses strength and speed";
pub const TRIBUTE_ELECTROCUTED: &str = "🌩️ {} is struck by lightning, loses health";
pub const TRIBUTE_FROZEN: &str = "🥶 {} suffers from hypothermia, loses speed.";
pub const TRIBUTE_OVERHEATED: &str = "🥵 {} suffers from heat stroke, loses speed.";
pub const TRIBUTE_DEHYDRATED: &str = "🌵 {} is severely dehydrated, loses strength";
pub const TRIBUTE_STARVING: &str = "🍴 {} is ravenously hungry, loses strength";
pub const TRIBUTE_POISONED: &str = "🧪 {} eats something poisonous, loses sanity";
pub const TRIBUTE_BROKEN_ARM: &str = "🦴 {} injures their arm, loses strength.";
pub const TRIBUTE_BROKEN_LEG: &str = "🦴 {} injures their leg, loses speed.";
pub const TRIBUTE_INFECTED: &str = "🤢 {} gets an infection, loses health and sanity";
pub const TRIBUTE_DROWNED: &str = "🏊 {} partially drowns, loses health and sanity";
pub const TRIBUTE_MAULED: &str = "🐾 {} is attacked by {} {}, takes {} damage!";
pub const TRIBUTE_BURNED: &str = "🔥 {} gets burned, loses health";
pub const TRIBUTE_HORRIFIED: &str = "😱 {} is horrified by the violence, loses {} sanity.";
pub const TRIBUTE_SUFFER: &str = "😭 {} suffers from loneliness and terror.";
pub const TRIBUTE_SELF_HARM: &str = "🤦 {} tries to attack themself!";
pub const TRIBUTE_SUICIDE: &str = "🪒 {} attempts suicide.";
pub const TRIBUTE_ATTACK_WIN: &str = "🔪 {} attacks {}, and wins!";
pub const TRIBUTE_ATTACK_WIN_EXTRA: &str = "🔪 {} attacks {}, and wins decisively!";
pub const TRIBUTE_ATTACK_WOUND: &str = "🤕 {} wounds {}";
pub const TRIBUTE_ATTACK_LOSE: &str = "🤣 {} attacks {}, but loses!";
pub const TRIBUTE_ATTACK_LOSE_EXTRA: &str = "🤣 {} attacks {}, but loses decisively!";
pub const TRIBUTE_ATTACK_MISS: &str = "😰 {} attacks {}, but misses!";
pub const TRIBUTE_ATTACK_KILL: &str = "☠️ {} is killed by {}";
pub const TRIBUTE_ATTACK_KILLED: &str = "☠️ {} successfully kills {}";
pub const TRIBUTE_ATTACK_HIDDEN: &str = "🤔 {} can't attack {}, they're hidden";
pub const TRIBUTE_DIES_FROM_STATUS: &str = "💀 {} dies from {}";
pub const TRIBUTE_DIES_FROM_EVENT: &str = "💀 {} dies by {}";
pub const TRIBUTE_ALREADY_DEAD: &str = "‼️ {} is already dead!";
pub const TRIBUTE_DEAD: &str = "❗️ {} is dead!";
pub const WEAPON_BREAK: &str = "🗡️ {} breaks their {}";
pub const SHIELD_BREAK: &str = "🛡️ {} breaks their {}";
pub const SPONSOR_GIFT: &str = "🎁 {} receives a(n) {} ({}x {} +{})";

// Area messages
pub const AREA_EVENT: &str = "=== ⚠️ A(n) {} has occurred in {} ===";
pub const TRAPPED_IN_AREA: &str = "💥 {} is trapped in the {}.";
pub const DIED_IN_AREA: &str = "🪦 {} died in the {}.";
pub const AREA_OPEN: &str = "=== 🔔 The {} is habitable again ===";

/// Formats a message with the given arguments
pub fn format_message(message: &str, args: Vec<String>) -> String {
    let mut formatted_message = message.to_string();
    for arg in args {
        let x = formatted_message.find("{}").unwrap();
        let range = x..x+2;
        formatted_message.replace_range(range, &arg);
    }
    formatted_message
}

pub fn print_message(message: &str, args: Vec<String>) {
    let formatted_message = format_message(message, args);
    println!("{}", formatted_message);
}