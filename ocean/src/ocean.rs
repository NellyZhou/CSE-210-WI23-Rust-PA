use crate::beach::Beach;
use crate::prey::{Algae, Clam, Minnow, Shrimp};
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::vec_deque::{Iter, VecDeque};

#[derive(Debug)]
pub struct Ocean {
    // TODO: Fill in fields here.
    beaches: VecDeque<Beach>,
    reefs: VecDeque<Rc<RefCell<Reef>>>,

}

impl Ocean {
    pub fn new() -> Ocean {
        let beaches = VecDeque::new();
        let reefs = VecDeque::new();
        Ocean {beaches: beaches, reefs: reefs}
    }

    pub fn add_beach(&mut self, beach: Beach) {
        self.beaches.push_back(beach)
    }

    pub fn beaches(&self) -> Iter<Beach> {
        self.beaches.iter()
    }

    pub fn reefs(&self) -> Iter<Rc<RefCell<Reef>>> {
        self.reefs.iter()
    }

    /**
     * Generate a reef with the specified number of each concrete type of prey, and then add it to the ocean.
     *   - Minnows should have a speed of 25.
     *   - Shrimp should have an energy of 1.
     *
     * Returns a reference to the newly created reef.
     */
    pub fn generate_reef(
        &mut self,
        n_minnows: u32,
        n_shrimp: u32,
        n_clams: u32,
        n_algae: u32,
    ) -> Rc<RefCell<Reef>> {
        let mut reef = Reef::new();
        for i in 0..n_minnows {
            let minnow = Minnow::new(25);
            reef.add_prey(Box::new(minnow));
        }
        for i in 0..n_shrimp {
            let shrimp = Shrimp::new(1);
            reef.add_prey(Box::new(shrimp));
        }
        for i in 0..n_clams {
            let clams = Clam::new();
            reef.add_prey(Box::new(clams));
        }
        for i in 0..n_algae {
            let algae = Algae::new();
            reef.add_prey(Box::new(algae));
        }
        let reef_ref = Rc::new(RefCell::new(reef));
        self.reefs.push_back(Rc::clone(&reef_ref));
        reef_ref
    }
}
