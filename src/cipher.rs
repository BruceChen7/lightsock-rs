use rand::prelude::SliceRandom;

#[derive(Debug)]
struct Cipher {
    password: Vec<u8>,
    decripted_passord: Vec<u8>,
}

impl Cipher {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let words: Vec<u8>;
        loop {
            let mut w: Vec<u8> = (0..255).collect();
            w.shuffle(&mut rng);
            let mut same = false;
            for item in w.iter().enumerate() {
                let (i, v) = item;
                if i == *v as usize {
                    same = true;
                    break;
                }
            }
            if !same {
                words = w;
                break;
            }
        }

        let mut decripted_passord = vec![0; 256];
        for item in words.iter().enumerate() {
            let (i, p) = item;
            decripted_passord[*p as usize] = i as u8;
        }
        Self {
            password: words,
            decripted_passord,
        }
    }

    fn encripted(&self, data: &Vec<u8>) -> Vec<u8> {
        data.into_iter()
            .map(|x| (self.password[*x as usize] as u8))
            .collect()
    }

    fn decripted(&self, data: &Vec<u8>) -> Vec<u8> {
        data.into_iter()
            .map(|&x| (self.decripted_passord[x as usize]))
            .collect()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_cipher() {
        use super::Cipher;
        let cipher = Cipher::new();
        let body = &vec![1, 2, 3];
        let encripted = cipher.encripted(body);
        let decripted = cipher.decripted(&encripted);
        assert_eq!(*body, decripted)
    }
}
