//specifying parent modules
/*use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    //--snip--
}

fn function2() -> io::Result<()> {
    //--snip--
}*/

// as keyword
/*use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    //--snip--
}

fn function2() -> IoResult<()> {
    //--snip--
}*/

//nesting paths
//these two lines can become...
use std::cmp::Ordering;
use std::io;
//this
use std::{cmp::Ordering, io};

//further these can be combined into 1
use std::io;
use std::io::Write;
//:
use std::io::{self, Write}; //this brings io and io::Write into scope

//the glob operator can bring all public items defined into scope
use std::collections::*;

