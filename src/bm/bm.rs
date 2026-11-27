use std::{cmp::max, collections::HashMap};

#[derive(
    PartialEq, 
    Eq, 
    PartialOrd, 
    Ord
)]
pub enum BM{
    BadCharRule,
    GoodSuffixRule
}

pub struct BMH;
pub struct BMHS;

/* NOTA: Essa função tem como objetivo, transformar os índices, antes
 * saídos de um vetor, projetados para as strings com índices variádicos
 * por causa do UTF-8. 
 * 
 * NOTA: Essa função possui complexidade O(n), mas
 * essa complexidade é "engolida" pela complexidade 
 * O(n * m) dos algoritmos.
 */
pub fn to_str_idx(
    mut res: Vec<usize>, 
    txt: &Vec<char>
) -> Vec<usize> {
    if res.is_empty() || txt.is_empty() {return res}
    let mut acct = 0;
    let mut res_idx = 0;
    for txt_idx in 0..txt.len() {
        if txt_idx == res[res_idx] {
            res[res_idx] = acct;
            res_idx += 1;
        }
        if res_idx >= res.len() {break;}
        acct += txt[txt_idx].len_utf8();
    } 
    res  
}

impl BM {
    fn bad_char_rule_table(pattern: &Vec<char>) -> HashMap<char, i64> {
        let mut table = HashMap::new();
        
        for ch in pattern {
            table.insert(*ch, 0);
        }


        for idx in 0..pattern.len()-1 {
            let _ = table.insert(pattern[idx], idx as i64);
        }

        table
    }

    fn bad_char_rule_search(
        txt: &str, 
        pattern: &str
    ) -> Vec<usize>{
        let mut res = vec![];

        if 
            txt.is_empty() ||
            pattern.is_empty()
        {return res;}

        let txt: Vec<char> = txt.chars().collect();
        let pattern: Vec<char> = pattern.chars().collect(); 

        let table = Self::bad_char_rule_table(&pattern);
        let mut start = 0;

        while start + pattern.len() <= txt.len() {
            let slice = 
                &txt[start..start + pattern.len()];

            let mut match_pattern = true;

            for idx in (0..pattern.len()).rev() {
                if 
                    slice[idx] != pattern[idx]
                {
                    match_pattern = false;

                    let jmp = 
                        if let Some(jmp) = table.get(&slice[idx]) {
                            idx as i64 - *jmp 
                        }
                        else {idx as i64 + 1};

                    start += max(1, jmp) as usize;

                    break;
                }
            }

            if match_pattern {
                res.push(start);
                start += 1;
            }
        }
        to_str_idx(res, &txt)
    }


    fn good_suffix_rule_search(
        txt: &str, 
        pattern: &str
    ) -> Vec<usize>{
        vec![]
    }

    pub fn search(
        &self,
        txt: &str, 
        pattern: &str
    ) -> Vec<usize>{
        match self {
            Self::BadCharRule => Self::bad_char_rule_search(txt, pattern),
            Self::GoodSuffixRule => Self::good_suffix_rule_search(txt, pattern)
        }
    }
}

impl BMH{
    fn table(pattern: &Vec<char>) -> HashMap<char, usize>{
        let mut table = HashMap::new();
        let size = pattern.len();

        for char in pattern {
            if table.contains_key(char) {continue;}
            table.insert(*char, size); 
        }

        for idx in 0..pattern.len()-1 {
            let Some(content) = table.get_mut(&pattern[idx]) else{
                unreachable!()
            };

            *content = size - 1 - idx;
        }

        table
    }

    pub fn search(
        txt: &str, 
        pattern: &str
    ) -> Vec<usize>{
        let mut res = vec![];

        if 
            txt.is_empty() ||
            pattern.is_empty()
        {return res;}

        let txt: Vec<char> = txt.chars().collect();
        let pattern: Vec<char> = pattern.chars().collect();

        let table = Self::table(&pattern);
        let mut start = 0;

        while 
            start + pattern.len() <= txt.len() 
        {
            let mut is_same = true;
            let slice = 
                &txt[start..start + pattern.len()];
            for idx in (0..pattern.len()).rev() {

                if pattern[idx] != slice[idx] {
                    is_same = false;
                    start += 
                        if let Some(jmp) = table.get(&slice[pattern.len()-1]) {
                            *jmp
                        }
                        else {
                            pattern.len()
                        };
                    break;
                }
            };
            if is_same {
                res.push(start);
                start += 1;
            }
        }
        to_str_idx(res, &txt)
    }
}

impl BMHS {
    fn table(pattern: &Vec<char>) -> HashMap<char, usize>{
        let mut table = HashMap::new();
        
        for ch in pattern{
            table.insert(*ch, pattern.len() + 1);
        }

        for idx in 0..pattern.len() {
            table.insert(pattern[idx], pattern.len() - idx);
        }
        
        table
    }
    pub fn search(
        txt: &str, 
        pattern: &str
    ) -> Vec<usize>{
        let mut res = vec![];

        if 
            txt.is_empty() || 
            pattern.is_empty() ||
            pattern.len() > txt.len()
        {return res}
        
        let txt: Vec<char> = txt.chars().collect();
        let pattern: Vec<char> = pattern.chars().collect();
        
        let table = Self::table(&pattern);
        
        let mut start = 0;

        while start <= txt.len() - pattern.len() {
            let txt_slice = 
                &txt[start..start+pattern.len()];
            let mut is_equal = true;
            
            for idx in 0..pattern.len() {
                let txt_ch = txt_slice[idx];
                

                if txt_ch != pattern[idx] {
                    is_equal = false;
                    if start + pattern.len() >= txt.len() {return to_str_idx(res, &txt)}

                    let last = 
                        txt[start + pattern.len()];
                    
                    start += 
                        if let Some(jmp) = table.get(&last) {
                            *jmp
                        }
                        else {
                            pattern.len() + 1
                        };
                    
                    break;
                }
            }

            if is_equal {
                res.push(start);
                if start + pattern.len() >= txt.len() {return to_str_idx(res, &txt)}

                let last = 
                    txt[start + pattern.len()];
                
                start += 
                    if let Some(jmp) = table.get(&last) {
                        *jmp
                    }
                    else {
                        pattern.len() + 1
                    };
            }
        }

        to_str_idx(res, &txt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buscar_bm_ocorrencia(txt: &str, pattern: &str) -> Vec<usize> {
        BM::BadCharRule.search(txt, pattern)
    }

    fn caso_exemplo_base(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("BMACBACBMACBMCBM", "CBM"), vec![6, 10, 13]);
    }

    fn caso_ocorrencia_unica_no_inicio(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("ALGORITMO DE BUSCA", "ALGORITMO"), vec![0]);
    }

    fn caso_ocorrencia_unica_no_meio(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("ESTRUTURA DE DADOS", "DE"), vec![10]);
    }

    fn caso_ocorrencia_unica_no_fim(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("XABC", "ABC"), vec![1]);
        assert_eq!(busca("ESTRUTURA DE DADOS", "DADOS"), vec![13]);
    }

    fn caso_texto_exatamente_igual_ao_padrao(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("PADRAO", "PADRAO"), vec![0]);
    }

    fn caso_padrao_inexistente_no_texto(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("ABCDEFGHIJKLMNOP", "XYZ"), Vec::<usize>::new());
        assert_eq!(busca("ABABABAB", "ABAC"), Vec::<usize>::new());
    }

    fn caso_padrao_maior_que_o_texto(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("CURTO", "TEXTO MUITO MAIOR"), Vec::<usize>::new());
    }

    fn caso_texto_vazio(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("", "ABC"), Vec::<usize>::new());
    }

    fn caso_padrao_de_um_unico_caractere(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("BANANA", "A"), vec![1, 3, 5]);
        assert_eq!(busca("BANANA", "B"), vec![0]);
        assert_eq!(busca("BANANA", "Z"), Vec::<usize>::new());
    }

    fn caso_texto_e_padrao_de_um_caractere(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("A", "A"), vec![0]);
        assert_eq!(busca("A", "B"), Vec::<usize>::new());
    }

    fn caso_multiplas_ocorrencias_sem_sobreposicao(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("ABC---ABC---ABC", "ABC"), vec![0, 6, 12]);
        assert_eq!(busca("ABCABCABC", "ABC"), vec![0, 3, 6]);
    }

    fn caso_ocorrencias_com_sobreposicao(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("ABABA", "ABA"), vec![0, 2]);
        assert_eq!(busca("AAAAA", "AA"), vec![0, 1, 2, 3]);
    }

    fn caso_exemplo_classico_boyer_moore(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(
            busca("GCATCGCAGAGAGTATACAGTACG", "GCAGAGAG"),
            vec![5]
        );
    }

    fn caso_falha_apos_casamento_parcial_do_sufixo(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("BBCAAXYZ", "BCAA"), vec![1]);
        assert_eq!(busca("ABAAAXYZ", "BAAA"), vec![1]);
        assert_eq!(busca("XBABABXYZ", "ABAB"), vec![2]);
        assert_eq!(busca("AAABAAAABXYZ", "AAAAB"), vec![4]);
    }

    fn caso_caractere_da_falha_a_direita_no_padrao(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("XXBBCBCXYZ", "BBCBC"), vec![2]);
        assert_eq!(busca("CABABXYZ", "ABAB"), vec![1]);
        assert_eq!(busca("ABCBABCBXYZ", "BCBAB"), vec![1]);
    }

    fn caso_caracteres_repetidos_quase_casamento_no_final(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("AAAAAAAAAB", "AAAAB"), vec![5]);
    }

    fn caso_diferencia_maiusculas_e_minusculas(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(
            busca("algoritmo ALGORITMO Algoritmo", "ALGORITMO"),
            vec![10]
        );
    }

    fn caso_caracteres_unicode_multibyte(busca: fn(&str, &str) -> Vec<usize>) {
        assert_eq!(busca("ação reação coração", "ação"), vec![0, 9, 19]);
        assert_eq!(busca("aaáabc", "abc"), vec![4]);
    }

    macro_rules! gerar_testes_busca {
        ($mod_name:ident, $search_fn:expr) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn exemplo_base() {
                    caso_exemplo_base($search_fn);
                }

                #[test]
                fn ocorrencia_unica_no_inicio() {
                    caso_ocorrencia_unica_no_inicio($search_fn);
                }

                #[test]
                fn ocorrencia_unica_no_meio() {
                    caso_ocorrencia_unica_no_meio($search_fn);
                }

                #[test]
                fn ocorrencia_unica_no_fim() {
                    caso_ocorrencia_unica_no_fim($search_fn);
                }

                #[test]
                fn texto_exatamente_igual_ao_padrao() {
                    caso_texto_exatamente_igual_ao_padrao($search_fn);
                }

                #[test]
                fn padrao_inexistente_no_texto() {
                    caso_padrao_inexistente_no_texto($search_fn);
                }

                #[test]
                fn padrao_maior_que_o_texto() {
                    caso_padrao_maior_que_o_texto($search_fn);
                }

                #[test]
                fn texto_vazio() {
                    caso_texto_vazio($search_fn);
                }

                #[test]
                fn padrao_de_um_unico_caractere() {
                    caso_padrao_de_um_unico_caractere($search_fn);
                }

                #[test]
                fn texto_e_padrao_de_um_caractere() {
                    caso_texto_e_padrao_de_um_caractere($search_fn);
                }

                #[test]
                fn multiplas_ocorrencias_sem_sobreposicao() {
                    caso_multiplas_ocorrencias_sem_sobreposicao($search_fn);
                }

                #[test]
                fn ocorrencias_com_sobreposicao() {
                    caso_ocorrencias_com_sobreposicao($search_fn);
                }

                #[test]
                fn exemplo_classico_boyer_moore() {
                    caso_exemplo_classico_boyer_moore($search_fn);
                }

                #[test]
                fn falha_apos_casamento_parcial_do_sufixo() {
                    caso_falha_apos_casamento_parcial_do_sufixo($search_fn);
                }

                #[test]
                fn caracteao_de_um_unico_caracterere_da_falha_a_direita_no_padrao() {
                    caso_caractere_da_falha_a_direita_no_padrao($search_fn);
                }

                #[test]
                fn caracteres_repetidos_quase_casamento_no_final() {
                    caso_caracteres_repetidos_quase_casamento_no_final($search_fn);
                }

                #[test]
                fn diferencia_maiusculas_e_minusculas() {
                    caso_diferencia_maiusculas_e_minusculas($search_fn);
                }
                 
                #[test]
                fn caracteres_unicode_multibyte() {
                    caso_caracteres_unicode_multibyte($search_fn);
                }
                
            }
        };
    }

    gerar_testes_busca!(bm_ocorrencia, buscar_bm_ocorrencia);
    gerar_testes_busca!(bmh, BMH::search);
    gerar_testes_busca!(bmhs, BMHS::search);
}
