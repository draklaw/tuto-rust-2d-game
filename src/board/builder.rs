use std::rc::Rc;

use rand::seq::SliceRandom;

use crate::config::AppConfig;
use crate::world::GameWorld;


pub struct Builder {
    config: Rc<AppConfig>,
}


impl Builder {
    pub fn new(config: &Rc<AppConfig>) -> Builder {
        let config = config.clone();
        Builder{ config }
    }

    
    pub fn build_on(&self, world: &mut GameWorld) {
        world.board.reset(&self.config.board_dim)
            .expect("valid dimension");

        // TODO error handling
        // TODO find tile sets compatible with `dim`
        let mut rng = rand::thread_rng();
        let tile_set = self.config.tile_sets.choose(&mut rng)
            .expect("config has at least one tile sets");
        tile_set.build_rand(&mut world.board)
            .expect("board can be build with tile sets");
    }
}
