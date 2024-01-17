use katana_primitives::block::Block;
use katana_primitives::state::StateUpdatesWithDeclaredClasses;

pub mod blockifier;

pub trait StarknetProcessor {
    fn execute(&self);
    fn execute_many(&self);
    fn estimate_fee(&self);
    fn call(&self);
}

pub trait BlockExecutor: StarknetProcessor {
    fn execute_block(&mut self, block: &Block);
    fn take_output(&mut self) -> StateUpdatesWithDeclaredClasses;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
