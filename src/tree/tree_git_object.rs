struct Tree {
  name:String,
  hash:String,
  blob:Vec<Blob>,
  tree: Vec<Tree>,
}

struct Blob {
  name:String,
  hash:String,
}