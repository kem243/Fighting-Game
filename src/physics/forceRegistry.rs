use crate::physics::particle; // used to apply gravity
use crate::physics::vecmath::PhysVec;

// Structs 
// Keeps track of one force generator and the particle it applies to.
#[derive(Debug)]
pub struct ParticleForceRegistration {
	pub Rc<RefCell<Particle>>: particle, // Add particle class from Carly
    pub ParticleForceGenerator: fg,
}
pub struct ParticleForceGenerator {
	pub gravity:  PhysVec::new(0f32, 9.8f32),
}

// Holds the list of registrations.
let Registry: Vec<ParticleForceRegistration> = Vec::new();

impl <'t> ParticleForceRegistration <'t> {
    /**
    * Registers the given force generator to apply to the
    * given particle.
    */
	pub fn add(Particle* particle, ParticleForceGenerator *fg) {
        Registry.push(particle.borrow(), fg);
	} 
    /**
    * Removes the given registered pair from the registry.
    * If the pair is not registered, this method will have
    * no effect.
    */
    pub fn remove(Particle* particle, ParticleForceGenerator *fg) {
        let i = 0;
        for item in Registry {
            if item.particle.borrow() == particle.borrow() && item.fg == fg{
                Registry.remove(i);
            } else{
                i = i + 1;
            }
        }
	}
    /**
    * Clears all registrations from the registry. This will
    * not delete the particles or the force generators
    * themselves, just the records of their connection.
    */
    pub fn clear() {
        Registry.clear();
	}
    /**
    * Calls all the force generators to update the forces of
    * their corresponding particles.
    */
    pub fn ParticleForceRegistry::updateForces() {
        //Registry::iterator i = registrations.begin();
        for item in Registry {
            item.fg.updateForce(item.particle.borrow());
        }
    }
} // close ParticleForceRegistration impl

impl <'t> ParticleForceGenerator <'t> {
    /**
    * Registers the given force generator to apply to the
    * given particle.
    */
	pub fn ParticleGravity::updateForce(Particle* particle) {
        let zero = PhysVec::new(0f32, 0f32);
        // Apply the mass-scaled force to the particle.
        particle.add_force(gravity * (1f32/particle.inverse_mass));
    }

    pub fn ParticlePunch::updateForce(Particle* particle) {
        // Apply the mass-scaled force to the particle.
        particle.add_force(gravity * (1f32/particle.inverse_mass));
    }

    pub fn ParticleKick::updateForce(Particle* particle) {
        // Apply the mass-scaled force to the particle.
        particle.add_force(gravity * (1f32/particle.inverse_mass));
    }
} // close ParticleForceGenerator impl