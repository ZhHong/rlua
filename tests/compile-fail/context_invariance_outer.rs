extern crate rlua;

use rlua::Lua;

fn main() {
    Lua::new().scope(|lua1| {
        let t = lua1.create_table().unwrap();
        //~^ error: cannot infer an appropriate lifetime for lifetime parameter `'lua` due to
        // conflicting requirements
        Lua::new().scope(|lua2| {
            lua2.globals().set("t", t).unwrap();
        });
    });
}
