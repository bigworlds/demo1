use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;
use ggez::nalgebra as na;
use ggez::input::keyboard::{self, KeyCode};
use rand::{self, thread_rng, Rng};

const RACKET_H: f32 = 100.;
const RACKET_W: f32 = 20.;
const RACKET_W_HALF: f32 = RACKET_W * 0.5;
const RACKET_H_HALF: f32 = RACKET_H * 0.5;
const BALL_SIZE: f32 = 30.;
const BALL_SIZE_H : f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.;
const BALL_SPEED: f32 = 10.;
const PADDING: f32 = 40.;
const MIDDLE_LINE_W: f32 = 2.;

struct MainState {
    player_1_pos: glam::Vec2,
    player_2_pos: glam::Vec2,
    ball_pos: glam::Vec2,
    ball_vel: glam::Vec2,
    racket_mesh: graphics::Mesh,
    player_1_score: i32,
    player_2_score: i32,
    duration: f32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w*0.5, screen_h*0.5);
        let racket_rect = graphics::Rect::new(-RACKET_W_HALF, -RACKET_H_HALF, RACKET_W, RACKET_H);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, graphics::WHITE).unwrap();
        let mut ball_vel = glam::Vec2::new(0., 0.);
        rand_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_pos : glam::Vec2::new(RACKET_W_HALF, screen_h_half), 
            player_2_pos : glam::Vec2::new(screen_w - RACKET_W_HALF, screen_h_half),
            ball_pos : glam::Vec2::new(screen_w_half, screen_h_half),
            racket_mesh,
            player_1_score : 0,
            player_2_score : 0,
            ball_vel,
            duration: 0.,
        }
    }
    
}

fn move_racket(pos: &mut glam::Vec2, keycode: KeyCode, y_dir: f32, dt: f32, ctx: &mut Context){
    let screen_h = graphics::drawable_size(ctx).1;
    if keyboard::is_key_pressed(ctx, keycode){
        pos.y += y_dir * PLAYER_SPEED * dt;
    }
    
}

fn rand_vec(vec : &mut glam::Vec2, x: f32, y: f32){
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5){
        true => x,
        false => -x,
    };

    vec.y = match rng.gen_bool(0.5){
        true => y,
        false => -y,
    };
    vec.normalize();
}

fn sdSphere(x: glam::Vec2, pos: glam::Vec2, rad : f32) -> f32 {
    let dist_2_surf: f32 = (x - pos).length() - rad;
    dist_2_surf
}

fn sdBox(x: glam::Vec2, pos: glam::Vec2, ext: glam::Vec2) -> f32{
    let dist = x - pos;
    let distVec = dist.abs() - ext;
    let maxdist = distVec.max(glam::Vec2::new(0.,0.));
    //let dist_2_surf = maxdist.min(glam::Vec2::new(0.,0.)) + (distVec.max(glam::Vec2::new(0.,0.)).length());
    let dist_2_surf = distVec.max(glam::Vec2::new(0., 0.)).length() + distVec.max_element().min(0.);
    dist_2_surf
}

fn sdScene(x: glam::Vec2, screen_w: f32, screen_h: f32) -> f32{
    let box_up_centre = glam::Vec2::new(screen_w * 0.5, 0. - (screen_h * 0.5));
    let box_up_ext = glam::Vec2::new(screen_w * 0.5, screen_h * 0.5);
    let box_down_centre = glam::Vec2::new(screen_w * 0.5, (screen_h+ (screen_h * 0.5)));
    let box_down_ext = glam::Vec2::new(screen_w * 0.5, screen_h * 0.5);
    let dist = sdBox(x, box_up_centre, box_up_ext);
    let min_dist = dist.min(sdBox(x, box_down_centre, box_down_ext));
    min_dist
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult{
        //dt
        let dt: f32 = 1./60.;
        self.duration += ggez::timer::delta(ctx).as_secs_f32();
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        while self.duration > dt {
            self.duration -= dt;
            
            move_racket(&mut self.player_1_pos, KeyCode::W, -1., dt, ctx);
            move_racket(&mut self.player_1_pos, KeyCode::S, 1., dt, ctx);
            move_racket(&mut self.player_2_pos, KeyCode::Up, -1., dt, ctx);
            move_racket(&mut self.player_2_pos, KeyCode::Down, 1., dt, ctx);
            self.ball_pos = self.ball_pos + self.ball_vel * BALL_SPEED * dt;
            
            //racket collision
            self.player_1_pos.y = self.player_1_pos.y.clamp(RACKET_H_HALF, screen_h - RACKET_H_HALF);
            self.player_2_pos.y = self.player_2_pos.y.clamp(RACKET_H_HALF, screen_h - RACKET_H_HALF);


            //ball collision
            //리셋 조건
            //sdScene이 meshid를 같이 리턴하게 만들어야 할듯
            if self.ball_pos.x < (0. + BALL_SIZE_H) {
                self.ball_pos.x = screen_w * 0.5;
                self.ball_pos.y = screen_h * 0.5;
                rand_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
                self.player_2_score += 1;
            }
            if self.ball_pos.x > (screen_w - BALL_SIZE_H) {
                self.ball_pos.x = screen_w * 0.5;
                self.ball_pos.y = screen_h * 0.5;
                rand_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
                self.player_1_score += 1;
            }

            //바운스
            let d = sdScene(self.ball_pos, screen_w, screen_h) - BALL_SIZE_H;
            if d < 0.{
                self.ball_vel.y = -self.ball_vel.y;
            }

            //라켓충돌
            let d1 = sdBox(self.ball_pos, self.player_1_pos, glam::Vec2::new(RACKET_W_HALF, RACKET_H_HALF)) - BALL_SIZE_H;
            if d1 < 0.{
                self.ball_vel.x = -self.ball_vel.x;
            }

            let d2 = sdBox(self.ball_pos, self.player_2_pos, glam::Vec2::new(RACKET_W_HALF, RACKET_H_HALF)) - BALL_SIZE_H;
            if d2 < 0.{
                self.ball_vel.x = -self.ball_vel.x;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult{
        //rendering
        graphics::clear(ctx, graphics::BLACK);

        let ball_mesh = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), ggez::nalgebra::Point2::new(0., 0.), BALL_SIZE_H, 0.1, graphics::Color::from_rgb(255, 255, 0))?;

        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = na::Point2::new(self.player_1_pos.x, self.player_1_pos.y).into();
        graphics::draw(ctx, &self.racket_mesh, draw_param)?;
        draw_param.dest = na::Point2::new(self.player_2_pos.x, self.player_2_pos.y).into();
        graphics::draw(ctx, &self.racket_mesh, draw_param)?;
        draw_param.dest = na::Point2::new(self.ball_pos.x, self.ball_pos.y).into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        let score_text = ggez::graphics::Text::new(format!("{}    {}", self.player_1_score, self.player_2_score));
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        let mut score_pos = na::Point2::new(screen_w_half, 40.);
        let (score_text_w, score_text_h) = score_text.dimensions(ctx);
        score_pos -= na::Vector2::new(score_text_w as f32*0.5, score_text_h as f32 * 0.5);
        draw_param.dest = score_pos.into();
        graphics::draw(ctx, &score_text, draw_param)?;

        let middle_rect = graphics::Rect::new(-MIDDLE_LINE_W*0.5, 0., MIDDLE_LINE_W, screen_h);
        let middle_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), middle_rect, graphics::WHITE)?;
        draw_param.dest = [screen_w_half, 0.].into();
        graphics::draw(ctx, &middle_mesh, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}
fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("01", "BIGWORLDS");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "PingPong");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state);

    Ok(())
}
