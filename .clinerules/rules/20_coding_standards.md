# Coding Standards

* The app is written in Rust and follows with a particular set of idioms. 
* When writing or editing code, these idioms must be followed consistently to
  keep the code quality high.
* If you encounter code that does not meet these standards, suggest (or strongly
  recommend) that it be brought up to these standards.

## Code Organization

### Individual Files for Individual Types

* Each type (`struct`s and `enum`s) must be in its own separate file, with very
  few exceptions.
* The file must be named after the type and converted to `snake_case`.  For
example, a type named `FooBar` must be in a file named `foo_bar.rs`.

### `README.md` Files

Code is to be organized into hierarchical directories corresponding to Rust
modules following Rust and Cargo conventions.  Each directory must have a
Markdown-formatted `README.md` file that gives an overview of what that
directory contains and its subdirectories.

* These files must have (relative) links to the `README.md` files of the
    subdirectories when describing them.
* They must also have a link to the `README.md` of the parent directory to ease
    navigation when, for example, reading them on GitHub.
* The `README.md` files must focus on broad descriptions of the abstractions in
    the module, but must not focus on details.
  * The details are in the code-level documentation.  Instead, t
  * The `README.md` files must focus on the file structure and how to navigate
      the codebase.
* These files must be automatically kept up-to-date as sub-directories
  (sub-modules) are added and/or removed or their contents are changed.
  * This must be part of your plan when in "Plan Mode".
    * Do not update these files repeatedly with every small step in a task
    * Update them once at the end of the task
  * When updating these files, you must also inspect the `README.md` in their
    parent directory to determine if that parent needs to be updated.
    * If it does, you must perform the update and then check *its* parent, etc.,
      recursively up the directory tree to the root `README.md` file. 

Here's a very brief example just to illustrate the structure.  The real
`README.md` files will have more fulsome documentation.

```markdown
[Parent](../README.md)

# Data Model

This package contains the internal data model.  All data from external sources
is transformed into this canonical format, and all of the data analysis is
performed on this model.

# Sub-Modules

* [`org`](org/README.md): Structures for representing motorsport organizations,
  their subdivisions, the events they hold, etc.
* [`geo`](geo/README.md): Primitives for working with geospatial data, such as
  Latitude and Longitude.
* [`telemetry`](telemetry/README.md): Abstractions for all of the supported
  telemetry data.
* [`store`](store/README.md): The interface for the backend persistent store for
  data.
* [`format`](format/README.md): On-disk formats that can be imported from and,
  in some cases, exported to.
* [`device`](device/README.md): Devices that can capture the imported telemetry
  data.  These map the external data formats that can be imported to the
  internal data model.
```

## Code Quality

Here is an example of a hypothetical `Measurement` type, which I will go through
in detail below:

```rust
```

### Documentation

* Banners for sections
* Module docs - For individual struct/enum files must describe the abstraction
from a user point of view, for modules with submodules/re-exports must
* Struct docs - must explain the abstract
* Doctests must be included.
* Private methods, test assertions, and test functions are documented as well
  (developers will generate code documentation with private methods and test
  assertions included)

### Imports

### Struct/Enum Definition

### Impl Blocks

### Unit Testing

Assertions - impl Into, references, lifetimes
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
Fine-grained
Tarpaulin - 100% coverage, must be part of writing unit tests