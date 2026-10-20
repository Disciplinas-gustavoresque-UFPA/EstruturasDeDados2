use std::mem::replace;


#[derive(Clone)]
pub(crate) struct RaddixNode{
    key: String,
    childrens: Vec<Box<RaddixNode>>
}

pub struct RaddixTree{
    root: Vec<RaddixNode>
}

impl RaddixNode{
    pub(crate) fn new(key: String) -> Self{
        Self { 
            key, 
            childrens: Vec::new()
        }
    }

    pub(crate) fn insert(&mut self, key: String){
        if key.len() == 0 {return;}

        let mut key_iter = key.chars();
        let mut key_idx = 0;

        let key_clone = self.key.clone();
        for (ch_idx, ch) in key_clone.chars().enumerate(){
            let key_ch_opt = key_iter.next();
            
            if let Some(key_ch) = key_ch_opt && key_ch != ch {
                let key_remainder = (&key[key_idx..]).to_string();
                
                for idx in 0..self.childrens.len(){
                    let children = self.childrens[idx].as_mut();
                
                    /* NOTA: Isso pode ser feito pois há a garantia
                     * de que se um filho existir ele não será uma 
                     * string vazia.
                     */
                    if children.key.chars().next().unwrap() == key_ch {
                        children.insert(key_remainder);
                        return;
                    }
                }

                self.childrens.push(Box::new(RaddixNode::new(key_remainder)));
                return;

            }

            /* NOTA: Se key_ch for None então devemos
             * criar um nó a com o resto da key atual.
             */
            else if key_ch_opt.is_none() {
                let mut new_children = Box::new(RaddixNode::new((&self.key[ch_idx..]).to_string()));
                new_children.childrens = replace(&mut self.childrens, Vec::new());
                
                let self_key_remainder= (&self.key[..ch_idx]).to_string();
                let _ = replace(&mut self.key,self_key_remainder);
                
                self.childrens.push(new_children);
                return;
            }

            key_idx += 1;
        }   

        if self.key.len() >= key.len() { return; }
        
        let key_remainder = (&key[self.key.len()..]).to_string();
                
        for idx in 0..self.childrens.len(){
            let children = self.childrens[idx].as_mut();
            /* NOTA: Isso pode ser feito, pois há a garantia
            * de que se um filho existir ele não será uma 
            * string vazia.
            */
            if children.key.chars().next().unwrap() == key_iter.next().unwrap(){
                children.insert(key_remainder);
                return;
            }
        }

        self.childrens.push(Box::new(RaddixNode::new(key_remainder)));
    }

    pub(crate) fn search(&self, key: String) -> bool{
        let mut key_node_iter = key.chars();
        let mut key_node_opt;
        for (ch_idx, ch) in self.key.chars().enumerate(){
                key_node_opt = key_node_iter.next();
                if let Some(key_ch) = key_node_opt {
                    if ch == key_ch {continue;}
                    
                    for idx in 0..self.childrens.len(){
                        let children = self.childrens[idx].as_ref();

                        if children.key.chars().next().unwrap() != key_ch {continue;}

                        return children.search((&key[ch_idx..]).to_string());
                    }
                }            
                else{
                    return false;
                }
                
        }
        
        
        if key.len() > self.key.len(){
            let key_ch = key_node_iter.next().unwrap();
            for idx in 0..self.childrens.len(){
                let children = self.childrens[idx].as_ref();

                if children.key.chars().next().unwrap() != key_ch {continue;}

                return children.search((&key[self.key.len()..]).to_string());
            }
        }

        self.key == key
    }

}

impl RaddixTree{
    pub fn new() -> Self{
        Self { root: Vec::new() }
    }

    pub fn from_vec(vec: &Vec<String>) -> Self{
        let mut raddix_tree = RaddixTree::new();
        for key in vec{
            raddix_tree.insert(key.clone());
        }
        raddix_tree
    }

    pub fn from_str_vec<'a>(vec: &Vec<&'a str>) -> Self{
        let mut raddix_tree = RaddixTree::new();
        for key in vec{
            raddix_tree.insert(key.to_string());
        }
        raddix_tree
    }

    pub fn insert(&mut self, key: String){
        if key.len() == 0 {return;}
        for idx in 0..self.root.len(){
            let children = &mut self.root[idx];

            if children.key.chars().next().unwrap() == key.chars().next().unwrap(){
                children.insert(key);
                return;
            }

        }

        self.root.push(RaddixNode::new(key));

    }

    pub fn search(&self, key: String) -> bool{
        if key.len() == 0 {return false;}

        for idx in 0..self.root.len(){
            let children = &self.root[idx];

            if children.key.chars().next().unwrap() == key.chars().next().unwrap(){
                return children.search(key);
            }
        }

        false
    }

}


#[macro_export]
macro_rules! raddix_tree{
    ($($val:expr),* $(,)?) => {
        {
            let mut raddix_tree = RaddixTree::new();
            $(
                raddix_tree.insert($val.to_string());
            )*
            raddix_tree
        }
    };
}


#[cfg(test)]
mod test{
    use crate::{raddix_tree::raddix_tree::RaddixTree, raddix_tree};

    #[test]
    fn test_insert(){
        let vec = vec!["hello", "hell", "pode", "poder", "poderei"];
        let tree = RaddixTree::from_str_vec(&vec);
        for key in vec{
            assert!(tree.search(key.to_string()));
        }
    }

    #[test]
    fn test_search(){
        let tree = raddix_tree!("marcar", "marchar", "machado", "mato");
        
        assert!(
            tree.search("marcar".to_string()) &&
            tree.search("marchar".to_string()) &&
            tree.search("machado".to_string()) &&
            tree.search("mato".to_string()) 
        );

        assert!(
            !tree.search("manco".to_string()) &&
            !tree.search("machucar".to_string()) &&
            !tree.search("machados".to_string()) &&
            !tree.search("matos".to_string()) &&
            !tree.search("".to_string())
        )

    }

}

