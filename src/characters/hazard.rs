use crate::animation;
use crate::input;

use sdl2::rect::{Point, Rect}; // for hazard hitboxes
// maybe incorporate a
pub enum Hazards {
	Stalagtites, // <- we can add more as we go if we want
}

// Structs
pub struct Hazard {
    pub active: bool,
    pub falling: bool,
    pub hit: bool,
    pub fall_speed: i32,
    pub damage: i32,
    pub x_cord: i32,
    pub y_cord: i32,
}

impl Hazard {
	pub fn new(c: Hazards) -> Stalagtites {
		Hazard {
            active: false,
            falling: false,
            hit: false,
            fall_speed: 10, // idk something to start with
            damage: 5, // same as above ^^
            y_cord: 99, // some max height
            x_cord: 99, // potentially a randomly generated number
		}
    }

        // setters
        pub fn set_active(&mut self, b:bool ) -> bool {
            self.active = b;
            return true;
        }
        pub fn set_falling(&mut self, b:bool) -> bool {
            self.falling = b;
            return true;
        }
        pub fn set_hit(&mut self, b:bool) -> bool {
            self.hit = b;
            return true;
        }
        pub fn set_fallspeed(&mut self, i:i32) -> bool {
            self.fall_speed = i;
            return true;
        }
        pub fn set_damage(&mut self, i:i32) -> bool {
            self.damage = i;
            return true;
        }
        pub fn set_xcord(&mut self, i:i32 ) -> bool {
            self.x_cord = i;
            return true;
        }
        pub fn set_ycord(&mut self, i:i32 ) -> bool {
            self.y_cord = i;
            return true;
        }

        // getters
        pub fn get_active(&self) -> &mut String {
            &mut self.active
        }
        pub fn get_falling(&self) -> &mut String {
            &mut self.falling
        }
        pub fn get_hit(&self) -> &mut String {
            &mut self.hit
        }
        pub fn get_fallspeed(&self) -> &mut String {
            &mut self.fall_speed
        }
        pub fn get_damage(&self) -> &mut String {
            &mut self.damage
        }
        pub fn get_xcord(&self) -> &mut String {
            &mut self.x_cord
        }
        pub fn get_ycord(&self) -> &mut String {
            &mut self.y_cord
        }

        // WIP
        pub fn fall(&mut self) -> bool {
            while !self.hit {
                &mut self.y_cord = self.y_cord - 1; // can be changed to something else
                // update sprite
                if self.check_hit {
                    break;
                }
            }
            // handle the collision values here
            return true;
        }
        pub fn check_hit(&mut self) -> bool {
            if self.y_cord <= 0 { return true; } // if it hit the ground
            else if self.y_cord <= 0 { return true; } // if it hit a non player
            else if self.y_cord <= 0 { return true; } // if it hit a player
            else { return false; } //
        }
        pub fn hit_player_check(&mut self) -> bool {
            return false;
        }
        pub fn hit_stage_check(&mut self) -> bool {
            return false;
        }
        pub fn assign_damage(&mut self) -> bool {
            return false;
        }
}
