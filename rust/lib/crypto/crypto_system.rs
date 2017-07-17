/// A crypto_system is a five-tuple (P,C,_KS_,E,D), where the following conditions are satisfied:
///
/// 1. P is a finite set of possible plaintexts;
/// 2. C is a finite set of possible ciphertexts;
/// 3. _KS_, the keyspace, is a finite set of possible keys;
/// 4. For each K IN _KS_, there is an encryption rule e_K in E and a corresponding decryption rule d_K in D. Each e_K : P -> C and d_K : C -> P are functions such that d_K(e_K(x)) = x for every plaintext element x IN P
///
/// ok!
pub trait CryptoSystem {
  type P; // plaintext
  type C; // ciphertext
  type K; // key
  fn new(&Self::K) -> Self;
  fn encrypt(&self, Vec<Self::P>) -> Vec<Self::C>;
  fn decrypt(&self, Vec<Self::C>) -> Vec<Self::P>;
}



pub trait KeySpace {
  type K;
  // TODO FIXME: Turn into iterator
  fn new(Self) -> Self;
  fn key_space(&self) -> u64;
  fn min_bound(&self) -> Option<&Self::K>;
  fn max_bound(&self) -> Option<&Self::K>;
}


pub fn crypto_system() {
}