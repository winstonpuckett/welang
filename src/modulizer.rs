use std::{
    collections::HashMap,
    fs::{self, metadata, read_dir},
};

use crate::{
    lexer::TokenContext,
    parser::{parse_module, Module},
};

#[derive(Debug, Clone)]
// pub enum Tree {
//     Internal(HashMap<String, Box<Tree>>),
//     Leaf(String, Module)
// }
pub struct Tree {
    pub module: Option<Module>,
    pub sub: Option<HashMap<String, Box<Tree>>>,
}
impl Tree {
    pub fn is_leaf(&self) -> bool {
        self.sub.is_none()
    }
}

pub fn to_module_tree(path: &str) -> Tree {
    let md = metadata(&path);
    match md {
        Ok(val) if val.is_file() => {
            let content = fs::read_to_string(&path);

            match content {
                Ok(source) => Tree {
                    module: Some(parse_module(&mut TokenContext::new(&source))),
                    sub: None,
                },
                Err(_) => todo!("Could not read {}", &path),
            }
        }
        Ok(val) if val.is_dir() => {
            let map = read_dir(path)
                .expect(&format!("Could not read dir: {}", &path))
                .map(|p| {
                    let file_name = p.unwrap().file_name();
                    let next_path = file_name.to_str().unwrap();
                    let module_name = get_mod_name(&next_path);
                    let submodule = to_module_tree(next_path);

                    (module_name, Box::new(submodule))
                })
                .collect::<HashMap<_, _>>();

            Tree {
                module: None,
                sub: Some(map),
            }
        }
        Ok(_) => todo!(), // could be a symlink

        Err(msg) => todo!("Could not modularize path {}, {}", &path, msg),
    }
}

fn get_mod_name(path: &str) -> String {
    // TODO: Windows style pathing / no unwraps
    path.split('.')
        .next()
        .unwrap()
        .to_owned()
        .split('/')
        .last()
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::to_module_tree;

    #[test]
    pub fn can_parse_full_syntax() {
        let tree = to_module_tree("./test/full.we");
        println!("{:#?}", tree);
    }
}
