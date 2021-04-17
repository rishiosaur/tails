use rltk::{GameState, Rltk};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "What up awefwaef");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Tails").build()?;
    let gs = State {};
    rltk::main_loop(context, gs)
}
