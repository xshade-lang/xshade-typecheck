use xshade_parser::*;

#[derive(Debug, Eq, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeId {
    id: usize,
}

impl TypeId {
    pub fn new(id: usize) -> TypeId {
        TypeId {
            id
        }
    }
}

#[derive(Debug)]
pub struct TypeSystem {
    types: Vec<TypeDefinition>,
}

impl TypeSystem {
    pub fn new() -> TypeSystem {
        TypeSystem {
            types: Vec::new(),
        }
    }

    pub fn create_type(&mut self, name: &str, location: Location, declaration: Declaration) -> TypeId {
        let id = self.types.len();

        self.types.push(TypeDefinition::new(
            id,
            name.to_string(),
            location,
            declaration
        ));

        TypeId::new(id)
    }

    pub fn find_type_by_type_id(&self, type_id: TypeId) -> Option<&TypeDefinition> {
        if self.types.len() > type_id.id {
            None
        } else {
            Some(&self.types[type_id.id])
        }
    }

    pub fn find_type_by_location(&self, location: &Location) -> Option<&TypeDefinition> {
        unimplemented!()
    }
}

// File & Span where the type was declared
#[derive(Debug)]
pub struct Declaration {
    path: String,
    span: Span,
}

impl Declaration {
    pub fn new(path: String, span: Span) -> Declaration {
        Declaration {
            path,
            span,
        }
    }
}

// Logical location (module system) of the type
#[derive(Debug)]
pub struct Location {
    path: String,
}

impl Location {
    pub fn new(path: String) -> Location {
        Location {
            path,
        }
    }
}

#[derive(Debug)]
pub struct TypeDefinition {
    id: usize,
    name: String,
    location: Location,
    declaration: Declaration,
}

impl TypeDefinition {
    pub fn new(id: usize, name: String, location: Location, declaration: Declaration) -> TypeDefinition {
        TypeDefinition {
            id,
            name,
            location,
            declaration,
        }
    }
}
