struct BanchoMod {
    binary_id: i32, // TODO verify if this is i32 and if it's an ID
    name_short: String,
    name_long: String,
}

impl BanchoMod {
    fn new(binary_id: i32, name_short: String, name_long: String) -> Self {
        BanchoMod {
            binary_id,
            name_short,
            name_long,
        }
    }
}
