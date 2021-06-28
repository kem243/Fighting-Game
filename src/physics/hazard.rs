extern crate street_code_fighter;

use sdl2::rect::{Point, Rect}; // for hazard hitboxes
// maybe incorporate a
pub enum Hazards {
	Stalactites, // <- we can add more as we go if we want
}

// Structs
pub struct Hazard {
    pub active: bool,
    pub falling: bool,
    pub hit: bool,
    pub fall_speed: f64,
    pub damage: f64,
    pub position: Point,
    pub collision: CollisionObject,
}

impl Hazard {
	pub fn new(c: Hazards) -> Hazard {
		Hazard {
            active: false,
            falling: false,
            hit: false,
            fall_speed: 1.0, // idk something to start with
            damage: 5.0, // same as above ^^
            position: Point::new(0,0),
            collision: Hazard::new(),
		}
    }

        // setters
        pub fn set_active(&mut self, b:bool ) -> bool { self.active = b; }
        pub fn set_falling(&mut self, b:bool) { self.falling = b; }
        pub fn set_hit(&mut self, b:bool) { self.hit = b; }
        pub fn set_fallspeed(&mut self, i:f64) { self.fall_speed = i; }
        pub fn set_damage(&mut self, i:f64) { self.damage = i; }
        pub fn set_xcord(&mut self, i:f64 ) { self.x_cord = i; }
        pub fn set_ycord(&mut self, i:f64 ) { self.y_cord = i; }

        // getters
        pub fn active(&self) -> bool { self.active }
        pub fn falling(&self) -> bool { self.falling }
        pub fn hit(&self) -> bool { self.hit }
        pub fn fallspeed(&self) -> f64 { self.fall_speed }
        pub fn damage(&self) -> f64 { self.damage }
        pub fn xcord(&self) -> &Point { self.position.x() }
        pub fn ycord(&self) -> &Point { self.position.y() }

        // methods
        pub fn fall(&mut self) -> bool {
            while !self.hit {
                let change = self.position.y() - self.fall_speed; // can be changed to something else
                self.fall_speed = self.fallspeed * 1.25; // accelerate fall speed
                self.position.offset(0, change);
                // update sprite
                if self.check_hit {
                    break;
                }
                self.fall_speed = self.fall_speed + 1;
            }
            // handle the collision values here
            return true;
        }
        // pub fn check_hit(&mut self) -> bool {
        //     if self.position.y() <= 0 { return true; } // if it hit the ground (assumed y level 0)
        //     else if self.position.y() <= 0 { return true; } // if it hit a non player
        //     else if self.position.y() <= 0 { return true; } // if it hit a player
        //     else { return false; } //
        // }
}
