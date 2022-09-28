use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "Welcome to the game!");
        ctx.print_centered(8, "Press [Enter] to start.");
        ctx.print_centered(9, "Press [Esc] to quit.");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Return => self.mode = GameMode::Playing,
                VirtualKeyCode::Escape => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(10, "Playing the game!");
        ctx.print_centered(11, "Press [Esc] to quit.");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => self.mode = GameMode::End,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "Thanks for playing!");
        ctx.print_centered(8, "Press [P] to Play Again.");
        ctx.print_centered(19, "Press [Esc] to quit.");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Escape => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello, world!")
        .build()?;
    main_loop(context, State::new())
}
