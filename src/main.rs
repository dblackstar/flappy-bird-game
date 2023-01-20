
use bracket_lib::prelude::*;

//-------------- availables game mode
enum GameMode {
    Menu,
    Playing,
    End,
}
//---------------------- CONSTANTS
const SCREEN_WIDTH:i32 = 80;
const SCREEN_HEIGTH:i32 = 50;
const FRAME_DURATION:f32 = 75.0;

//---------------------- play
struct Player {
    x:i32,
    y:i32,
    velocity:f32,   // speed
}

impl Player {
    
    // create a new player instance
    fn new(x:i32, y:i32)->Self{
        Player { x: x, y: x, velocity: 0.0 }
    }

    // render the player on left screen
    fn render(&mut self, ctx:&mut BTerm){
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }

    // gravity
    fn gravity_and_move(&mut self){
        if self.velocity < 2.0 {
            self.velocity +=0.2;
        }
        
        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0{
            self.y = 0;
        }
    }

    // flap wings
    fn flap(&mut self){
        self.velocity = -2.0;
    }

}


//---------------------- Obstacles in the game ==> Wll
struct Obstacle{
    x:i32,
    gap_y:i32,
    size:i32,
}
// obstacles methods
impl Obstacle {
    
    // create a new obstable
    fn new(x:i32, score:i32)->Self{
        let mut random = RandomNumberGenerator::new();
        Obstacle { 
            x:x, 
            gap_y: random.range(10, 40), 
            size: i32::max(2, 20-score),
        }
    }

    // obstacle rendering
    fn render(&mut self, ctx:&mut BTerm, player_x:i32){
        let screen_x = self.x - player_x;
        let half_size = self.size/2;

        // draw the top half of the obstacles
        for y in 0..self.gap_y - half_size{
            ctx.set(
                screen_x,
                y,
                RED,
                BLACK,
                to_cp437('|'),
            );
        }

        // draw the bottom half of the obstacles
        for y in self.gap_y + half_size..SCREEN_HEIGTH {
            ctx.set(
              screen_x,
              y,
              RED,
              BLACK,
              to_cp437('|'),
            );
        }

    }

    // crashing into the wall
    fn hit_obstacle(&self, player:&Player)->bool{
        let half_size = self.size/2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}



//---------------------- game state
struct State {
    player:Player,
    frame_time:f32,
    obstacle: Obstacle,
    mode:GameMode,
    score:i32,
}



// game state methods 
impl State {
    fn new()->Self{
        State { 
            player:Player::new(6, 25),
            frame_time:0.0,
            mode: GameMode::Menu, 
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score:0,
        }
    }

    // play
    fn play(&mut self, ctx:&mut BTerm){
               
        ctx.cls_bg(NAVY);
    self.frame_time += ctx.frame_time_ms;
    if self.frame_time > FRAME_DURATION {
      self.frame_time = 0.0;

      self.player.gravity_and_move();
    }
    if let Some(VirtualKeyCode::Space) = ctx.key {
      self.player.flap();
    }
    self.player.render(ctx);
    // START: play
    ctx.print(0, 0, "Press SPACE to flap.");
    ctx.print(0, 1, &format!("Score: {}", self.score)); // <callout id="co.flappy.printscore" />

    self.obstacle.render(ctx, self.player.x); // <callout id="co.flappy.obstaclerender" />
    if self.player.x > self.obstacle.x { // <callout id="co.flappy.scoreup" />
      self.score += 1;
      self.obstacle = Obstacle::new(
          self.player.x + SCREEN_WIDTH, self.score
      );
    }
    if self.player.y > SCREEN_HEIGTH || 
        self.obstacle.hit_obstacle(&self.player)
    {
      self.mode = GameMode::End;
    }
    }

    // restart
    fn restart(&mut self){
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.mode = GameMode::Playing;
        self.score = 0;
    }

    // main menu
    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5 ,"Welcome to flappy dragon");
        ctx.print_centered(10 ,"(P) Play game");
        ctx.print_centered(13 ,"(Q) Quit game");

        if let Some(key) = ctx.key {
            match key{
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting =true,
                _=>{}
            }
        }
    }

    // Dead
    fn dead(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        // END: dead
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::P => self.restart(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
        }
        }
    }

}
// trait for game state
impl GameState for State {

    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

// main function
fn main() -> BError {
    
    // create the context (window)
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    // call the main loop
    main_loop(context,State::new())
}
