use std::{collections::BTreeMap, ops::Deref};

type NestedSectionTree = BTreeMap<String, SectionTreeLeaf>;

#[derive(Debug)]
pub struct SectionTree(NestedSectionTree);

impl SectionTree {
    pub fn parse(text: &str) -> Option<SectionTree> {
        let keypairs: Vec<(usize, &str, Option<&str>)> = parse_to_keypairs(text)?;
        Some(SectionTree(build_tree(&keypairs, 0)))
    }
}

impl Deref for SectionTree {
    type Target = NestedSectionTree;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug)]
pub enum SectionTreeLeaf {
    Text(String),
    Tree(NestedSectionTree)
}

fn build_tree(keypairs: &[(usize, &str, Option<&str>)], level: usize) -> NestedSectionTree {
    let mut root = BTreeMap::new();

    for (_indent, key, value_opt) in keypairs.iter().filter(|(indent, _, _)| *indent == level) {
        if let Some(value) = value_opt {
            root.insert(key.to_string(), SectionTreeLeaf::Text(value.to_string()));
        }
        else {
            root.insert(key.to_string(), SectionTreeLeaf::Tree(build_tree(keypairs, level + 1)));
        }
    }

    root
}

fn parse_to_keypairs(text: &str) -> Option<Vec<(usize, &str, Option<&str>)>> {
    let mut vec: Vec<(usize, &str, Option<&str>)> = Default::default();

    for line in text.lines() {
        let indent = line.chars().take_while(|c| *c == '\t').count();
        let stripped = line.get(indent..)?.trim_ascii_end();
        if let Some(key) = stripped.strip_suffix(":") { // Tree 
            vec.push((indent, key, None));
        }
        else if let Some((key, value)) = stripped.split_once(": ") { // Text
            vec.push((indent, key, Some(value)));
        }
    }

    Some(vec)
}

#[cfg(test)]
mod tests {
    use crate::parse::section_tree::{SectionTree, SectionTreeLeaf};


    #[test]
    fn simple() {
        let text = "
Details:
\tMinecraft Version: 1.21.5
\tMinecraft Version ID: 1.21.5
\tOperating System: Windows 11 (amd64) version 10.0
\tFabric Mods: 
\t\tbadoptimizations: BadOptimizations 2.4.1
\t\tbalm: Balm 21.5.25
";
        let tree = SectionTree::parse(&text).expect("Failed to parse tree");
        let SectionTreeLeaf::Tree(details_node) = tree.get("Details").expect("Failed to get details node") else { panic!("Not a tree node"); };
        let SectionTreeLeaf::Text(mc_version_value) = details_node.get("Minecraft Version").expect("Failed to get mc version node") else { panic!("Not a text node"); };
        assert_eq!(mc_version_value, "1.21.5");
        dbg!(tree);
    }
}
