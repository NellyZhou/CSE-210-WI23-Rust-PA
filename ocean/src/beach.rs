use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    crabs: Vec<Crab>
}

impl Beach {
    pub fn new() -> Beach {
        let mut crabs = Vec::new();
        Beach {crabs: crabs}
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab)
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        if self.size()== 0 {
            Option::None
        } else  {
            let mut fast_crab = &self.crabs[0];
            for i in self.crabs() {
                if i.speed() > fast_crab.speed() {
                    fast_crab = i
                }
            }
            Option::Some(fast_crab)
        }
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut certain_crabs = Vec::new();
        for crab in self.crabs() {
            if crab.name() == name {
                certain_crabs.push(crab);
            }
        }
        certain_crabs
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        let mom = self.get_crab(i);
        let pap = self.get_crab(j);
        let new_crab = Crab::new(name, 1, Color::cross(mom.color(), pap.color()), Diet::random_diet());
        self.add_crab(new_crab)
    }
}
