#![windows_subsystem = "windows"]
use macroquad::prelude::*;
use std::env;
use std::f64::consts::PI;
use std::f64::consts::TAU;

const WINDOW_HEIGHT: f64 = 1024.0;
const WINDOW_WIDTH: f64 = 2048.0;
const PLAYER_SIZE: f32 = 8.0;
const P2: f64 = PI / 2.0;
const P3: f64 = 3.0 * PI / 2.0;
const DEGREE: f64 = 0.0174533;
const FOV: f64 = 90.0;
const RAY_DENSITY: f64 = WINDOW_WIDTH / FOV;
const GRID_SIZE: f64 = 16.0;
const SCALING_FACTOR: f32 =
    ((WINDOW_HEIGHT / SIMULATION_WINDOW_HEIGHT) * (SIMULATION_SIZE_SQRT / GRID_SIZE)) as f32;
const SIMULATION_SIZE_SQRT: f64 = 8.0; //don't touch
const SIMULATION_SIZE: f32 = 64.0; //don't touch
const SIMULATION_WINDOW_HEIGHT: f64 = 512.0; //don't touch

fn dist(a_x: f64, a_y: f64, b_x: f64, b_y: f64) -> f64 {
    ((b_x - a_x) * (b_x - a_x) + (b_y - a_y) * (b_y - a_y)).sqrt()
}

pub enum GameState {
    Debug,
    Normal,
}

struct Player {
    rect: Rect,
    angle: f64,
    delta_x: f64,
    delta_y: f64,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(100.0, 300.0, PLAYER_SIZE, PLAYER_SIZE),
            angle: 0.0,
            delta_x: 1.0,
            delta_y: 1.0,
        }
    }

    pub fn draw(&self) {
        let rect_center = self.rect.center();
        draw_rectangle(
            (rect_center.x - 2.0) * SCALING_FACTOR,
            (rect_center.y - 2.0) * SCALING_FACTOR,
            self.rect.w,
            self.rect.h,
            PURPLE,
        );

        draw_line(
            rect_center.x * SCALING_FACTOR,
            rect_center.y * SCALING_FACTOR,
            (rect_center.x + (self.delta_x as f32) * 20.0) * SCALING_FACTOR,
            (rect_center.y + (self.delta_y as f32) * 20.0) * SCALING_FACTOR,
            4.0,
            BLUE,
        )
    }

    pub fn update(&mut self, map: [[u32; GRID_SIZE as usize]; GRID_SIZE as usize]) {
        match (is_key_down(KeyCode::Right), is_key_down(KeyCode::Left)) {
            (true, false) => {
                self.angle += 0.03;
                if self.angle > TAU {
                    self.angle -= TAU;
                }
                self.delta_x = self.angle.cos();
                self.delta_y = self.angle.sin();
            }
            (false, true) => {
                self.angle -= 0.03;
                if self.angle < 0.0 {
                    self.angle += TAU;
                }
                self.delta_x = self.angle.cos();
                self.delta_y = self.angle.sin();
            }
            _ => {}
        };

        //collision detecttion
        let xo;
        if self.delta_x < 0.0 {
            xo = -10.0;
        } else {
            xo = 10.0;
        }
        let yo;
        if self.delta_y < 0.0 {
            yo = -10.0;
        } else {
            yo = 10.0;
        }

        let rect_center = self.rect.center();

        match (is_key_down(KeyCode::Down), is_key_down(KeyCode::Up)) {
            (true, false) => {
                if map[((rect_center.x - xo) / SIMULATION_SIZE) as usize]
                    [(rect_center.y / SIMULATION_SIZE) as usize]
                    == 0
                {
                    self.rect.x -= self.delta_x as f32;
                }
                if map[(rect_center.x / SIMULATION_SIZE) as usize]
                    [((rect_center.y - yo) / SIMULATION_SIZE) as usize]
                    == 0
                {
                    self.rect.y -= self.delta_y as f32;
                }
            }
            (false, true) => {
                if map[((rect_center.x + xo) / SIMULATION_SIZE) as usize]
                    [(rect_center.y / SIMULATION_SIZE) as usize]
                    == 0
                {
                    self.rect.x += self.delta_x as f32;
                }
                if map[(rect_center.x / SIMULATION_SIZE) as usize]
                    [((rect_center.y + yo) / SIMULATION_SIZE) as usize]
                    == 0
                {
                    self.rect.y += self.delta_y as f32;
                }
            }
            _ => {}
        }
    }

    pub fn draw_rays_3d(
        &self,
        map: [[u32; GRID_SIZE as usize]; GRID_SIZE as usize],
        should_draw_rays: bool,
    ) {
        let mut ray_y = 0.0;
        let mut ray_x = 0.0;
        let mut xo = 0.0;
        let mut yo: f64 = 0.0;
        let rect_center = self.rect.center();
        let mut dist_total = 0.0;
        let mut line_o;
        let true_ray_density = if should_draw_rays {
            RAY_DENSITY / 2.0
        } else {
            RAY_DENSITY
        };

        let mut rays_angle = self.angle - DEGREE * FOV / 2.0;
        if rays_angle < 0.0 {
            rays_angle += TAU;
        }
        if rays_angle > TAU {
            rays_angle -= TAU;
        }

        for i in 0..((FOV * true_ray_density) as i32) {
            //check horinzontal grid lines
            let a_tan = -1.0 / rays_angle.tan();

            let mut dof = 0.0;

            let mut dis_h: f64 = 1000000.0;
            let mut h_x: f64 = self.rect.x as f64;
            let mut h_y: f64 = self.rect.y as f64;

            if rays_angle > PI {
                //looking up
                // ray_y = ((self.rect.y as i64 >> 6) << 6) as f64 - 0.0001;
                ray_y = (((h_y as i64) / SIMULATION_SIZE as i64) * SIMULATION_SIZE as i64) as f64
                    - 0.0001;
                ray_x = (h_y - ray_y) * a_tan + h_x;
                yo = -SIMULATION_SIZE as f64;
                xo = -yo * a_tan;
            } else if rays_angle < PI {
                //looking down
                // ray_y = ((self.rect.y as i64 >> 6) << 6) as f64 + 64.0;
                ray_y = (((h_y as i64) / SIMULATION_SIZE as i64) * SIMULATION_SIZE as i64) as f64
                    + SIMULATION_SIZE as f64;
                ray_x = (h_y - ray_y) * a_tan + h_x;
                yo = SIMULATION_SIZE as f64;
                xo = -yo * a_tan;
            } else if rays_angle == 0.0 || rays_angle == PI {
                // looking straight left or right
                ray_x = h_x;
                ray_y = h_y;
                dof = GRID_SIZE;
            }

            while dof < GRID_SIZE {
                let map_x = (ray_x as usize) >> 6;
                let map_y = (ray_y as usize) >> 6;
                if map_x < GRID_SIZE as usize
                    && map_y < GRID_SIZE as usize
                    && map[map_x][map_y] == 1
                {
                    h_x = ray_x;
                    h_y = ray_y;
                    dis_h = dist(rect_center.x as f64, rect_center.y as f64, h_x, h_y);
                    dof = GRID_SIZE; //hit wall
                } else {
                    // next line
                    ray_x += xo;
                    ray_y += yo;
                    dof += 1.0;
                }
            }

            // check vertical lines
            let n_tan = -rays_angle.tan();

            let mut dof = 0.0;

            let mut dis_v: f64 = 1000000.0;
            let mut v_x: f64 = self.rect.x as f64;
            let mut v_y: f64 = self.rect.y as f64;

            if rays_angle > P2 && rays_angle < P3 {
                //looking left
                // ray_x = ((self.rect.x as i64 >> 6) << 6) as f64 - 0.0001;
                ray_x = (((v_x as i64) / SIMULATION_SIZE as i64) * SIMULATION_SIZE as i64) as f64
                    - 0.0001;
                ray_y = (v_x - ray_x) * n_tan + v_y;
                xo = -SIMULATION_SIZE as f64;
                yo = -xo * n_tan;
            }
            if rays_angle < P2 || rays_angle > P3 {
                //looking right
                // ray_x = ((self.rect.x as i64 >> 6) << 6) as f64 + 64.0;
                ray_x = (((v_x as i64) / SIMULATION_SIZE as i64) * SIMULATION_SIZE as i64) as f64
                    + SIMULATION_SIZE as f64;
                ray_y = (v_x - ray_x) * n_tan + v_y as f64;
                xo = SIMULATION_SIZE as f64;
                yo = -xo * n_tan;
            }
            if rays_angle == 0.0 || rays_angle == PI {
                // looking straight up or down
                ray_x = v_x;
                ray_y = v_y;
                dof = GRID_SIZE;
            }

            while dof < GRID_SIZE {
                let map_x = (ray_x as usize) >> 6;
                let map_y = (ray_y as usize) >> 6;
                if map_x < GRID_SIZE as usize
                    && map_y < GRID_SIZE as usize
                    && map[map_x][map_y] == 1
                {
                    v_x = ray_x;
                    v_y = ray_y;
                    dis_v = dist(rect_center.x as f64, rect_center.y as f64, v_x, v_y);
                    dof = GRID_SIZE; //hit wall
                } else {
                    // next line
                    ray_x += xo;
                    ray_y += yo;
                    dof += 1.0;
                }
            }

            let mut rect_color: Color = BLUE;

            if dis_v < dis_h {
                ray_x = v_x;
                ray_y = v_y;
                dist_total = dis_v;
                rect_color = GRAY
            } else if dis_h < dis_v {
                ray_x = h_x;
                ray_y = h_y;
                dist_total = dis_h;
                rect_color = LIGHTGRAY;
            }

            if should_draw_rays {
                draw_line(
                    rect_center.x * SCALING_FACTOR,
                    rect_center.y * SCALING_FACTOR,
                    (ray_x as f32) * SCALING_FACTOR,
                    (ray_y as f32) * SCALING_FACTOR,
                    2.0,
                    BLUE,
                );
            }
            // draw 3d scene
            let mut c_a = self.angle - rays_angle;
            if c_a < 0.0 {
                c_a += TAU;
            }
            if rays_angle > TAU {
                c_a -= TAU;
            }

            dist_total = dist_total * c_a.cos().abs(); // fix fisheye

            let max_height: f32 = WINDOW_HEIGHT as f32;
            let mut line_h: f32 = (SIMULATION_SIZE * max_height) / dist_total as f32;
            if line_h > max_height {
                line_h = max_height;
            }
            line_o = WINDOW_HEIGHT as f32 / 2.0 - line_h / 2.0;

            let wall_width;
            let wall_o;
            if should_draw_rays {
                wall_o = WINDOW_WIDTH as f32 / 2.0;
                wall_width = ((WINDOW_WIDTH / 2.0) / (FOV * true_ray_density)) as f32;
            } else {
                wall_o = 0.0;
                wall_width = (WINDOW_WIDTH / (FOV * true_ray_density)) as f32;
            }
            //walls
            draw_rectangle(
                i as f32 * wall_width + wall_o,
                line_o,
                wall_width,
                line_h,
                rect_color,
            );

            //floor
            draw_rectangle(
                i as f32 * wall_width + wall_o,
                line_o + line_h,
                wall_width,
                WINDOW_HEIGHT as f32 - (line_o + line_h),
                BROWN,
            );

            //ceiling
            draw_rectangle(
                i as f32 * wall_width + wall_o,
                0.0,
                wall_width,
                line_o,
                SKYBLUE,
            );
            rays_angle += DEGREE / true_ray_density;
            if rays_angle < 0.0 {
                rays_angle += TAU;
            }
            if rays_angle > TAU {
                rays_angle -= TAU;
            }
        }
    }
}

struct World {
    map: [[u32; GRID_SIZE as usize]; GRID_SIZE as usize],
}

impl World {
    pub fn new() -> Self {
        Self {
            //initializes and 8x8 grid
            // map: [
            //     [1, 1, 1, 1, 1, 1, 1, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 1, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 1, 1, 1, 1, 1, 1, 1],
            // ],
            // map: [
            //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 1, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            //     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            // ],
            map: [
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
        }
    }

    pub fn draw_map_2d(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                let color = if block == &1 { WHITE } else { BLACK };
                let size = (WINDOW_HEIGHT / GRID_SIZE) as f32;

                let xo = (x as f32) * size;
                let yo = (y as f32) * size;

                draw_rectangle(yo + 1.0, xo + 1.0, size - 1.0, size as f32 - 1.0, color);
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Raycaster".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new();
    let world = World::new();
    let mut game_state = GameState::Normal;

    for argument in env::args() {
        if argument == "debug" {
            game_state = GameState::Debug;
        }
    }

    loop {
        player.update(world.map);

        clear_background(GRAY);

        match game_state {
            GameState::Normal => {
                player.draw_rays_3d(world.map, false);
            }
            GameState::Debug => {
                world.draw_map_2d();
                player.draw();
                player.draw_rays_3d(world.map, true);
            }
        }

        // fps counter
        let fps_text = format!("{}", get_fps());
        draw_text_ex(
            &fps_text,
            WINDOW_WIDTH as f32 - 45.0,
            WINDOW_HEIGHT as f32 - 14.0,
            TextParams {
                font_size: 30u16,
                color: BLACK,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
