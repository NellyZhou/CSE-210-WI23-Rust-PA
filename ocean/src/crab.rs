use crate::color::Color;
use crate::cookbook::{Cookbook, Recipe};
use crate::diet::Diet;
use crate::prey::Prey;
use crate::reef::Reef;
use std::cell::{RefCell, Ref};
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct Crab {
    // TODO: Add fields here (some in part 1, some in part 2)
    name: String,
    speed: u32,
    color: Color,
    diet: Diet,
    reefs: Vec<Rc<RefCell<Reef>>>,
}

// Do NOT implement Copy for Crab.
impl Crab {
    pub fn new(name: String, speed: u32, color: Color, diet: Diet) -> Crab {
        let mut reefs = Vec::new();
        Crab {name, speed, color, diet, reefs: reefs}
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn diet(&self) -> Diet {
        self.diet
    }

    // PART 2 BELOW
    // ------------

    /**
     * Have this crab discover a new reef, adding it to its list of reefs.
     */
    pub fn discover_reef(&mut self, reef: Rc<RefCell<Reef>>) {
        self.reefs.push(Rc::clone(&reef))
    }

    /**
     * Returns Some prey from one of the reefs this crab feeds from,
     * and the index of that reef in self.reefs if able to find Some prey
     * using the `take_prey` method of Reef.
     *
     * If `take_prey` returns None, try the next reef. Try each reef only once.
     *
     * If all reefs are empty, or this crab has no reefs, return None.
     */
    fn catch_prey(&mut self) -> Option<(Box<dyn Prey>, usize)> {
        let mut i: usize = 0;
        for reef in self.reefs.iter() {
            let prey = reef.borrow_mut().take_prey();
            match prey {
                Some(x) => return Some((x, i)),
                None =>  i = i + 1
            }
        }
        None
    }

    /**
     * Releases the given prey back into the reef at the given index.
     */
    fn release_prey(&mut self, prey: Box<dyn Prey>, reef_index: usize) {
        let reef = &mut self.reefs[reef_index];
        reef.borrow_mut().add_prey(prey)
    }

    /**
     * Have this crab go hunting.
     *
     * A crab will keep trying to catch prey until it succeeds,
     * or runs out of remaining prey to try to catch.
     *
     * You should keep track of all escaped prey in a local.
     *
     * Once you have finished hunting, release all escaped prey back into
     * the reefs from whence they came _before_ you return if prey was caught.
     *
     * Your algorithm might look something like this pseudocode. The challenge
     * in this task is not intended to be in figuring out the algorithm, but
     * in figuring out _how to write it in Rust_ using what you've learned so far.
     *
     * ```text
     *     there are no escaped prey yet
     *     prey has not been caught
     *     while prey can be caught
     *       if prey escapes
     *         mark the prey as escaped
     *         try again
     *     
     *       if prey is not edible by this crab
     *         mark the prey as escaped
     *         try again
     *       
     *       prey has been caught
     *       stop trying
     *     
     *     release each escaped prey back to its reef
     *     was prey caught?
     * ```
     *
     * Note: this pseudocode reads like a terrible poem.
     */
    pub fn hunt(&mut self) -> bool {
        let mut escape_prey = VecDeque::new();
        let mut isCaught = false;
        while true {
            let prey_ref = self.catch_prey();
            match prey_ref {
                None => break,
                Some((mut prey, reef_index)) => {
                    let isEscape = prey.as_mut().try_escape(&self);
                    if isEscape || (prey.as_ref().diet() != self.diet()) {
                        escape_prey.push_back((prey, reef_index));
                        continue;
                    } else {
                        isCaught = true;
                        break
                    }
                }
            }
        }
        while !escape_prey.is_empty() {
            let res = escape_prey.pop_back();
            match res {
                None => break,
                Some((prey, reef_index))=> self.release_prey(prey, reef_index)
            }
        }
        isCaught
    }

    /**
     * Returns Some of any recipe from the given cookbook that matches the crab's diet
     * preferences, or None if no such recipe exists.
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    pub fn choose_recipe<'a> (& 'a self, cookbook: &'a Cookbook) -> Option<& 'a Recipe> {
        for r in cookbook.recipes() {
            if r.diet() == self.diet {
                return Some(r)
            }
        }
        None
    }
}
