use crossbeam_channel::Sender;
use rayon::prelude::*;

#[derive(Default)]
pub struct Container {
    items: Vec<f32>,
}

impl<'a> Container {
    pub fn new() -> Self {
        Container {
            items: vec![1.0, 2.0, 3.0, 4.0],
        }
    }
    pub fn find(&'a self, less_than: f32, sender: Sender<Vec<f32>>) {
        self.items
            .par_iter()
            .try_for_each_with(sender, |s, p| {
                if *p < less_than {
                    return s.send(vec![*p]);
                }
                Ok(())
            })
            .ok();
    }
}
