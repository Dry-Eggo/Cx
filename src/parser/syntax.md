## Syntax for the Cx programming language

### Comments
- Single-line comments start with `--` and continue to the end of the line. `--` over `//` Because it looks cooler.
```
-- This is a single-line comment
-- Another comment here
```

-- There are no multi-line comments in Cx. Only Docstrings.
```
### 
This is a docstring.
it can be used to document functions, structs, and other elements.
unlike comments, docstrings are attached to the AST and can be extracted by documentation tools (if any).
###
```

-- A program in Cx is a sequence of declarations. Declarations can be of various types, including:
* Function Declarations
* Struct Declarations
* Variable Declarations
* Constant Declarations
* Enum Declarations
* Import Declarations
* Module Declarations

### Function Declarations
- Functions are declared using the `fn` keyword, followed by the function name, parameters, return type, and body.
```
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // function body
}
```
very similar to Rust, but not exactly the same (or maybe it is).
- Parameters are defined within parentheses, with each parameter having a name and type.
- The return type is specified after the `->` symbol.
- The function body is enclosed in curly braces `{}`.

### Variable Declarations and Assignments
- Unlike Rust, Cx uses the `var` keyword for variable declarations.
```
var variable_name: Type = initial_value; -- the type can be ommited if it can be inferred
```
- Variables can be mutable by default, but you can use `const` for immutable variables.
```
const constant_name: Type = value;
```
- Variables can be assigned new values using the `=` operator.
```
variable_name = new_value;
```

### Struct Declarations
- Structs are declared using the `struct` keyword, followed by the struct name and body.
```
struct StructName {
    field1: Type1,
    field2: Type2,
}
```
- Methods are also supported within structs. The Method Syntax is similar to that of functions only that the first parameter must
  describe how `self` is taken
```
struct StructName {
    field1: Type1,
    field2: Type2,

    -- here, `self` is taken by a reference
    -- the reference is mutable
    -- this is similar to Rust's `&mut self`
    fn method_name(&self, param: Type) -> ReturnType {
        // method body
    }

    -- here, `self` is taken by a const reference
    -- it may not be modified by this method
    fn method_name_const(&const self, param: Type) -> ReturnType {
        // method body
    }

    -- here, `self` is taken by value, hence a 'Copy'
    -- any modifications here will not affect the original object
    fn method_name_copy(self, param: Type) -> ReturnType {
        // method body
    }

    -- here, `self` is taken by a pointer
    -- this is similar to C/C++'s `Type* self`
    fn method_name_ptr(*self, param: Type) -> ReturnType {
        // method body
        (*self).field1 = value;
    }

    -- here, `self` if also take by a pointer but to a const self this time.
    -- no modifications can be made to the value pointed to
    fn method_name_const_ptr(*const self) -> ReturnType {
        // method body
    }
}
```
