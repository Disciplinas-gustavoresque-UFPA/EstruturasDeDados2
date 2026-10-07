pub struct UnionFind{ //declarar a struct da dsu
    pai: Vec<usize>,  //vetor para guardar os índices dos nós pai de cada elemento

}

impl UnionFind{

    //tentar deixar esse construtor melhor depois
    pub fn new(n: usize) -> Self{ //n é o numero de elementos que vamos trabalhar

        let mut pai = Vec::with_capacity(n);

        //incializar cada elemento como seu próprio pai
        for i in 0..n{
            pai.push(i); 
        }

        UnionFind{ pai }

    }

    pub fn find(&self, i: usize) -> usize{
        if self.pai[i] == i{ 
            i
        } else {
            self.find(self.pai[i]) //se não for raiz, vai chamar a função de forma recursiva até achar uma raiz
        }
    }

    pub fn union(&mut self, x: usize, y: usize){
        //encontrar as raizes dos elementos:
        let raiz_x = self.find(x);
        let raiz_y = self.find(y);

        if raiz_x != raiz_y { //se não forem o mesmo conjunto
            self.pai[raiz_y] = raiz_x;
        }
    }

    pub fn is_same_set(&self, x: usize, y: usize) -> bool { 
        self.find(x) == self.find(y)
    }

    pub fn print_state(&self){
        println!("estrutura:");

        for i in 0..self.pai.len(){
            println!("elemento {}: pai - {}", i, self.pai[i]);
        }
    }
}



