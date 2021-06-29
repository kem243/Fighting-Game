use crate::animation; // to reference sprite State
use crate::input; // use to reference Direction

use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use std::collections::HashMap;

// Enums 
// defines optional Characters
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Characters {
	Python,
	// Stretch goal: add more
}

// Structs 
// defines the current state of the character
#[derive(Debug)]
pub struct CharacterState {
	pub position: Point,
    pub state: animation::sprites::State,
	pub frames_per_state: i32,
	pub current_frame: i32, 
	pub sprite: Rect,
	pub auto_repeat: bool,
	pub direction: input::movement::Direction,
	pub next_state: animation::sprites::State,	
}

// EDIT: consider updating integers to f64
pub struct Fighter<'t> {
	pub name: Characters,
	pub char_state: CharacterState, 
	pub speed: i32,
    pub weight: i32,
    pub gravity: f32,
    pub max_fall_speed: i32,
    pub walk_speed: i32,
    pub run_speed: i32,
    pub max_air_speed: i32,
    pub aerial_transition_speed: i32,
    pub crawl_speed: i32,
    pub dodge_speed: i32,
    pub friction: f32,
    pub static_grip: i32,
    pub pivot_grip: i32,
    pub air_resistance: f32,
    pub air_control: i32,
    pub jumps: i32,
    pub jump_height: i32,
    pub short_hop_height: i32,
    pub air_jump_height: i32,
    pub heavy_land_lag: i32,
    pub fastfall: i32,
    pub shield_size: i32,
  	pub textures: HashMap<animation::sprites::State, Texture<'t>>,

}

impl <'t> Fighter <'t> {
	pub fn new (c: CharacterState) -> Fighter<'t> {
		Fighter {
			name: Characters::Python,
			char_state: c,
			speed: 20, // arbitrary #
			weight: 180,
			gravity: -9.8,
			max_fall_speed: 20,
			walk_speed: 10,
			run_speed: 15,
			max_air_speed: 5,
			aerial_transition_speed: 3,
			crawl_speed: 3,
			dodge_speed: 5,
			friction: -0.1,
			static_grip: 20,
			pivot_grip: 25,
			air_resistance: -0.1,
			air_control: 5,
			jumps: 2,
			jump_height: 100,
			short_hop_height: 5,
			air_jump_height: 7,
			heavy_land_lag: 2,
			fastfall: 200,
			shield_size: 3,
      	textures: HashMap::new(),
		}
	} 
	
	// Getters
    pub fn weight(&self) -> &i32 {&self.weight}
    pub fn gravity(&self) -> &f32 {&self.gravity}
    pub fn max_fall_speed(&self) -> &i32 {&self.max_fall_speed}
    pub fn walk_speed(&self) -> &i32 {&self.walk_speed}
    pub fn run_speed(&self) -> &i32 {&self.run_speed}
    pub fn max_air_speed(&self) -> &i32 {&self.max_air_speed}
    pub fn aerial_transition_speed(&self) -> &i32 {&self.aerial_transition_speed}
    pub fn crawl_speed(&self) -> &i32 {&self.crawl_speed}
    pub fn dodge_speed(&self) -> &i32 {&self.dodge_speed}
    pub fn friction(&self) -> &f32 {&self.friction}
    pub fn static_grip(&self) -> &i32 {&self.static_grip}
    pub fn pivot_grip(&self) -> &i32 {&self.pivot_grip}
    pub fn air_resistance(&self) -> &f32 {&self.air_resistance}
    pub fn air_control(&self) -> &i32 {&self.air_control}
    pub fn jumps(&self) -> &i32 {&self.jumps}
    pub fn jump_height(&self) -> &i32 {&self.jump_height}
    pub fn short_hop_height(&self) -> &i32 {&self.short_hop_height}
    pub fn air_jump_height(&self) -> &i32 {&self.air_jump_height}
    pub fn heavy_land_lag(&self) -> &i32 {&self.heavy_land_lag}
    pub fn fastfall(&self) -> &i32 {&self.fastfall}
    pub fn shield_size(&self) -> &i32 {&self.shield_size} 

	pub fn textures(&self) -> &Texture<'t> {
		match &self.textures.get(&self.char_state.state) {
			Some(texture) => texture,
			None => panic!("Texture issue in fighter"),
		}
	}

	pub fn add_texture(&mut self, s: animation::sprites::State, t: Texture<'t>) {
            &self.textures.insert(s, t);
	}

    // Setters
    pub fn set_weight(&mut self) -> &mut i32 {&mut self.weight}
    pub fn set_gravity(&mut self) -> &mut f32 {&mut self.gravity}
    pub fn set_max_fall_speed(&mut self) -> &mut i32 {&mut self.max_fall_speed}
    pub fn set_walk_speed(&mut self) -> &mut i32 {&mut self.walk_speed}
    pub fn set_run_speed(&mut self) -> &mut i32 {&mut self.run_speed}
    pub fn set_max_air_speed(&mut self) -> &mut i32 {&mut self.max_air_speed}
    pub fn set_aerial_transition_speed(&mut self) -> &mut i32 {&mut self.aerial_transition_speed}
    pub fn set_crawl_speed(&mut self) -> &mut i32 {&mut self.crawl_speed}
    pub fn set_dodge_speed(&mut self) -> &mut i32 {&mut self.dodge_speed}
    pub fn set_friction(&mut self) -> &mut f32 {&mut self.friction}
    pub fn set_static_grip(&mut self) -> &mut i32 {&mut self.static_grip}
    pub fn set_pivot_grip(&mut self) -> &mut i32 {&mut self.pivot_grip}
    pub fn set_air_resistance(&mut self) -> &mut f32 {&mut self.air_resistance}
    pub fn set_air_control(&mut self) -> &mut i32 {&mut self.air_control}
    pub fn set_jumps(&mut self) -> &mut i32 {&mut self.jumps}
    pub fn set_jump_height(&mut self) -> &mut i32 {&mut self.jump_height}
    pub fn set_short_hop_height(&mut self) -> &mut i32 {&mut self.short_hop_height}
    pub fn set_air_jump_height(&mut self) -> &mut i32 {&mut self.air_jump_height}
    pub fn set_heavy_land_lag(&mut self) -> &mut i32 {&mut self.heavy_land_lag}
    pub fn set_fastfall(&mut self) -> &mut i32 {&mut self.fastfall}
    pub fn set_shield_size(&mut self) -> &mut i32 {&mut self.shield_size}
} // close Fighter impl

// Implementations
impl CharacterState {
	// initialize
	pub fn new() -> CharacterState {
		// current default values
		// Stretch goals: expand to not use default values
		CharacterState {
			position: Point::new(0,0),
			state: animation::sprites::State::Idle,
			frames_per_state: 5,
			current_frame: 0, 
			sprite: Rect::new(0, 0, 210, 300),
			auto_repeat: true,
			next_state: animation::sprites::State::Idle,
			direction: input::movement::Direction::Up,
		}
	}
	
	// update Point position
	pub fn update_position(&mut self, vel: i32, x_bounds: (i32, i32)){
		let x = (self.position.x() + vel).clamp(x_bounds.0, x_bounds.1);
		let current_y = self.position.y();
		self.position = Point::new(x, current_y);
	} 
	
    // advancing frames
    pub fn advance_frame(&mut self) { self.current_frame = (self.current_frame + 1) % self.frames_per_state; } // need to stay within # of frames

	// convenience f(x)	
	// getters
	pub fn position(&self)  	-> &Point 						{ &self.position } 
	pub fn state(&self)     	-> &animation::sprites::State 	{ &self.state }
	pub fn frames_per_state(&self) -> i32 						{ self.frames_per_state } // for testing
	pub fn current_frame(&self) -> i32 							{ self.current_frame } 
	pub fn sprite(&self) 		-> &Rect 						{ &self.sprite }
	pub fn auto_repeat(&self)	-> bool 						{ self.auto_repeat }
	pub fn next_state(&self) 	-> &animation::sprites::State 	{ &self.next_state }
	pub fn x(&self)				-> i32							{ self.position.x() }
	pub fn y(&self)				-> i32							{ self.position.y() }
	pub fn direction(&self)		-> &input::movement::Direction	{ &self.direction }
	
	// settters (use to update)
	pub fn set_position(&mut self, p: Point)						{ self.position = p; }
	pub fn set_state(&mut self, s: animation::sprites::State)		{ self.state = s; 
																	  self.frames_per_state = animation::sprites::get_frame_cnt(self);
																	  // println!("s: {:?}, cf: {}", self.state, self.current_frame);
																	}
	pub fn set_current_frame(&mut self, i: i32)						{ self.current_frame = (self.current_frame + i) % self.frames_per_state; } // need to stay within # of frames
	pub fn set_sprite(&mut self, r: Rect)							{ self.sprite = r; }
	pub fn set_auto_repeat(&mut self, b: bool)						{ self.auto_repeat = b; }
	pub fn set_next_state(&mut self, s: animation::sprites::State)	{ self.next_state = s; }
	pub fn set_direction(&mut self, d: input::movement::Direction)	{ self.direction = d; }
	pub fn reset_current_frame(&mut self)							{ self.current_frame = 0; }

	// pub fn isMoving(&self) -> bool {
	// 	if self.state == animation::sprites::State::Jump || self.state == animation::sprites::State::FJump 
	// 	|| self.state == animation::sprites::State::LPunch || self.state == animation::sprites::State::LKick
	// 	|| self.state == animation::sprites::State::HKick {
	// 		true
	// 	} else {
	// 		false
	// 	}
	// }

	pub fn isMoving(&self) -> bool {
		if self.state == animation::sprites::State::LPunch || self.state == animation::sprites::State::LKick
		|| self.state == animation::sprites::State::HKick {
			true
		} else {
			false
		}
	}

}