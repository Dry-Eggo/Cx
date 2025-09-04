

pub struct Module {
    // all string values are indexes into here
    string_pool: Vec<String>, 
}

// A Label can be of multiple types
// those types being:
// * external-label
// * global-label
// * function-label
// ... and so on
pub enum Label {
   ExternLabel (usize),
   GlobalLabel (usize),
   FunctionLabel {
       id: usize,
       builder: Builder,
   }
}

pub struct Builder {
    instructions: Vec<Instruction>
}

pub enum Instruction {
    
}
