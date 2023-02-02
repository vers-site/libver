pub struct INode {
    pub name: String,
    pub path: String,
    pub children: Vec<INode>,
}

pub struct DNode {
    pub name: String,
    pub path: String,
    pub children: Vec<DNode>,
}

pub struct FNode {
    pub name: String,
    pub path: String,
    pub children: Vec<INode>,
}
