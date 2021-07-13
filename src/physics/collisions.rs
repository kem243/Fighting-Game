#![allow(non_snake_case)]
use sdl2::rect::{Rect, Point};
use std::cell::{RefCell, Ref};
use std::ops::{Deref, DerefMut};
use std::fmt;
use crate::view::globals::*;
use crate::physics::nodes::*;
use crate::physics::particle::*;
use crate::physics::vecmath::*;

pub struct BVHierarchy {
	pub head: NodeRef<CollisionObject>,
}

impl BVHierarchy {
	pub fn new(co: CollisionObject) -> BVHierarchy {
		BVHierarchy{ head: NodeRef::new(co) }
	}
	pub fn insert(&self, co: CollisionObject) -> RefCell<CollisionObject> {
		// println!("inserting {:?}", co);
		self.head.insert(co)
	}
	pub fn resolve_collisions(&self) {
		let mut potential_collisions: Vec<ParticleContact> = Vec::new();
		self.head.getPotentialCollisions(&mut potential_collisions, 100);
		for contact in potential_collisions.iter_mut() {
			let p0 = contact.particles[0].clone();
			let p1 = contact.particles[1].clone();
			if check_collision(p0.borrow().clone(), p1.borrow().clone()) {
				let types = (p0.borrow().obj_type, p1.borrow().obj_type);
				match types {
					(CollisionObjectType::Platform, _) | (_, CollisionObjectType::Platform) => (),
					_ => println!("Contact between\n {:?}\nand\n {:?}", p0, p1),
				}
				contact.resolve_velocity(FRAME_RATE as f32);
			}
		}
	}
}

pub fn boxUp<T>(data: T) -> Option<RefCell<T>>{
	Some(RefCell::new(data))
}

pub struct Node<T> {
    pub parent: WeakLink<T>,
    pub left: Link<T>,
    pub right: Link<T>,
    pub bv: Option<RefCell<T>>, // bounding volume
	pub area: Rect, // total bounding area of children
}
impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("\n\tNode")
		.field("\n\tleft", &self.left)
		.field("\n\tright", &self.right)
		.field("\n\tbv", &self.bv)
		.finish()
    }
}

pub fn check_collision(a: CollisionObject, b: CollisionObject) -> bool {
	let types = (&a.obj_type, &b.obj_type);
	match types {
		(CollisionObjectType::HurtBox, CollisionObjectType::HurtBox) => false,
		_ => a.rect.has_intersection(b.rect.clone())
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CollisionObjectType {
	HitBox,
	HurtBox,
	BlockBox, // for if we want to implement it elsewhere
    Hazard,
    Platform,
    Wall,
	Empty,
}

pub struct ParticleContact {
	pub particles: Vec<RefCell<CollisionObject>>,
	pub restitution: f32, // idek what this does yet
	pub contact_normal: PhysVec,
	pub interpenetration: f32,
}

impl ParticleContact {
	pub fn new(p0:RefCell<CollisionObject>, p1: RefCell<CollisionObject>, restitution: f32, interpenetration: f32) -> Self {
		ParticleContact {
			particles: vec![p0, p1],
			restitution: restitution,
			contact_normal: PhysVec::new(0.0, 0.0),
			interpenetration: interpenetration,
		}
	}

	fn calculate_normal(&mut self) {

	}

	fn resolve_penetration(&self) {

	}

	fn separating_velocity(&self) -> f32 {
		let contact_0 = self.particles[0].borrow();
		let p0 = contact_0.particle.borrow();
		let contact_1 = self.particles[1].borrow();
		let p1 = contact_1.particle.borrow();
		let mut relative_velocity = p0.velocity.clone();
		relative_velocity.replace(&relative_velocity.sub(&p1.velocity));
		relative_velocity.scalar_product(&self.contact_normal)
	}

	fn resolve_velocity(&mut self, duration: f32) {
		let p0 = &self.particles[0].borrow().particle;
		let p1 = &self.particles[1].borrow().particle;
		let separating_velocity = self.separating_velocity();
		if separating_velocity > 0f32 { return } // contact is either separating or stationary, no impulse required

		let new_sep_velocity = -separating_velocity*self.restitution;
		let delta_velocity = new_sep_velocity - separating_velocity;

		let total_inv_mass = p0.borrow().inverse_mass + p1.borrow().inverse_mass;
		let impulse = delta_velocity / total_inv_mass;
		let impulse_per_mass = self.contact_normal.dot_product(impulse);

		match &self.particles[0].borrow().obj_type {
			CollisionObjectType::Platform => (),
			_ => {
				let v = p0.borrow().velocity.clone();
				let m = p0.borrow().inverse_mass;
				p0.borrow_mut().velocity.add_vec(&impulse_per_mass.dot_product(m));
			}
		}
		match &self.particles[1].borrow().obj_type {
			CollisionObjectType::Platform => (),
			_ => {
				let v = p1.borrow().velocity.clone();
				let m = p1.borrow().inverse_mass;
				p1.borrow_mut().velocity.add_vec(&impulse_per_mass.dot_product(m));
			}
		}
	}
}

pub trait Area {
	fn area(&self) -> u32;
}

impl Area for Rect {
	fn area(&self) -> u32 {
		self.width()*self.height()
	}
}

#[derive(Clone)]
pub struct CollisionObject {
    pub obj_type: CollisionObjectType,
	pub area: u32,
    pub rect: Rect,
	pub noderef: WeakLink<CollisionObject>,
	pub particle: RefCell<Particle>
}


impl fmt::Debug for CollisionObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("\n\t\tCollision Object")
		.field("obj_type", &self.obj_type)
		.field("area", &self.area)
		.field("rect", &self.rect)
		.field("position", &self.particle)
		.finish()
    }
}

impl CollisionObject {
    pub fn new(obj_type: CollisionObjectType, x: i32, y: i32, width: u32, height: u32, particle: RefCell<Particle>) -> CollisionObject {
        let rect = Rect::new(x, y, width, height);
		let area = rect.area();
		let noderef: WeakLink<CollisionObject> = None;

        CollisionObject {
            obj_type,
			area,
            rect,
			noderef,
			particle,
        }
    }
    pub fn new_from(obj_type: CollisionObjectType, rect: Rect, particle: RefCell<Particle>) -> CollisionObject {
		let area = rect.area();
		let noderef: WeakLink<CollisionObject> = None;

        CollisionObject {
            obj_type,
			area,
            rect,
			noderef,
			particle,
        }
    }
	pub fn getNodeRef(&self) -> Option<NodeRef<CollisionObject>> {
		match &self.noderef {
       		Some(p) => p.upgrade().map(|up| NodeRef(up)), //p.upgrade().map(|u| NodeRef(u)),
			None => None
		}
	}
	pub fn update(&mut self, position: Point) {
		self.rect.reposition(position);
	}

	fn overlapsWith(&self, other: &CollisionObject) -> bool {
		self.rect.has_intersection(other.rect)
	}
}

trait Unbox<T> {
	fn unbox<'a> (&'a self) -> &'a mut T;
}

impl Node<CollisionObject> {
	pub fn new(parent: WeakLink<CollisionObject>, bv: CollisionObject) -> Self {
		let area = bv.rect.clone();
		let bv = boxUp(bv);
		Node{
			parent: parent,
			left: None,
			right: None,
			bv: bv,
			area: area,
		}
	}

	pub fn isLeaf(&self) -> bool {
		!self.bv.is_none()
	}

	pub fn detatch(&mut self) {
		let parent = self.parent.take();
		let left = self.left.take();
		let right = self.right.take();
		let mut bv = self.bv.take();
		bv.take().map(|bv| bv.borrow_mut().noderef.take());
	}
}

/*
#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn testCollide() {
		let c1 = CollisionObject::new(CollisionObjectType::HitBox, 20, 20, 10, 20);
		let c2 = CollisionObject::new(CollisionObjectType::Hazard, 28, 20, 10, 20);

		assert_eq!(check_collision(&c1, &c2), true);
	}

	#[test]
	fn testCollisionInitFrom() {
		let r1 = Rect::new(0, 0, 3, 3);
		let c1 = CollisionObject::new_from(CollisionObjectType::HitBox, r1);

		assert_eq!(c1.rect, r1);
	}

	#[test]
	fn testCollisionUpdate() {
		let r1 = Rect::new(0, 0, 3, 3);
		let mut c1 = CollisionObject::new_from(CollisionObjectType::HitBox, r1);
		c1.update(Point::new(4, 4));

		assert_eq!(c1.rect, Rect::new(4,4,3,3));
	}

	#[test]
	fn testBVHNodeInit() {
		let co = CollisionObject::new(CollisionObjectType::HitBox, 0, 2, 3, 3);
		let node = NodeRef::new(co.clone());

		assert_eq!(node.get().left.as_ref().map(|a| Some(false)), None);
		assert_eq!(node.get().right.as_ref().map(|a| Some(false)), None);
		assert_eq!(node.get().bv.as_ref().take(), Some(&RefCell::new(co)));
		assert_eq!(node.get().area, Rect::new(0,2,3,3));
	}
	
	#[test]
	fn testBVHNodeInsert() {
		let co1 = CollisionObject::new(CollisionObjectType::HitBox, 0, 2, 3, 3);
		let co2 = CollisionObject::new(CollisionObjectType::HitBox, 5, 0, 6, 2);
		let co3 = CollisionObject::new(CollisionObjectType::HitBox, 20, 20, 2, 2);
		let node = NodeRef::new(co1.clone());
		let l = node.clone();
		let new = node.insert(co2.clone());

		assert_eq!(node.getLeftChild().get().bv.as_ref().unwrap(), &RefCell::new(co1.clone()));
		assert_eq!(node.getRightChild().get().bv.as_ref().unwrap(), &RefCell::new(co2.clone()));
		assert_eq!(node.get().bv.as_ref().take(), None);
		assert_eq!(node.get().area, Rect::new(0,0,11,5));

		node.insert(co3.clone());
		let l2 = NodeRef::new(co3);
		l2.getMut().parent = Some(std::rc::Weak::new());

		assert_eq!(node.getLeftChild().getLeftChild().get().bv.as_ref().unwrap(), &RefCell::new(co1.clone()));
		assert_eq!(node.getLeftChild().getRightChild().get().bv.as_ref().unwrap(), &RefCell::new(co3.clone()));
		assert_eq!(node.getRightChild().get().bv.as_ref().unwrap(), &RefCell::new(co2.clone()));
		assert_eq!(node.get().bv.as_ref().take(), None);
		assert_eq!(node.get().area, Rect::new(0,0,22,22));
	}
	
	// #[test]
	// fn testBVHNodeRemove() {
	// 	let co1 = CollisionObject::new(CollisionObjectType::HitBox, 0, 2, 3, 3);
	// 	let co2 = CollisionObject::new(CollisionObjectType::HitBox, 5, 0, 6, 2);
	// 	let co3 = CollisionObject::new(CollisionObjectType::HitBox, 20, 20, 2, 2);
	// 	let node = NodeRef::new(co1.clone());
	// 	node.insert(co2.clone());
	// 	let mut nodec3 = node.insert(co3.clone());
	// 	nodec3.remove();

	// 	assert_eq!(node.getLeftChild().get().bv.as_ref().unwrap(), &RefCell::new(co1.clone()));
	// 	assert_eq!(node.getRightChild().get().bv.as_ref().unwrap(), &RefCell::new(co2.clone()));
	// 	assert_eq!(node.get().bv.as_ref().take(), None);
	// 	assert_eq!(node.get().area, Rect::new(0,0,11,5));
	// }
}
*/
