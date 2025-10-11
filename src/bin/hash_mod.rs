// Custom hash implementation without external libraries
use std::mem;

// Custom hasher trait
trait Hashable {
    fn hash(&self) -> u64;
}

// FNV-1a hash implementation (simple and fast)
fn fnv1a_hash(bytes: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;
    
    let mut hash = FNV_OFFSET;
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

// DJB2 hash implementation (alternative)
fn djb2_hash(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for &byte in bytes {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

// Implement Hashable for primitive types
impl Hashable for i32 {
    fn hash(&self) -> u64 {
        let bytes = self.to_le_bytes();
        fnv1a_hash(&bytes)
    }
}

impl Hashable for i64 {
    fn hash(&self) -> u64 {
        let bytes = self.to_le_bytes();
        fnv1a_hash(&bytes)
    }
}

impl Hashable for u32 {
    fn hash(&self) -> u64 {
        let bytes = self.to_le_bytes();
        fnv1a_hash(&bytes)
    }
}

impl Hashable for u64 {
    fn hash(&self) -> u64 {
        let bytes = self.to_le_bytes();
        fnv1a_hash(&bytes)
    }
}

impl Hashable for f64 {
    fn hash(&self) -> u64 {
        let bytes = self.to_le_bytes();
        fnv1a_hash(&bytes)
    }
}

impl Hashable for String {
    fn hash(&self) -> u64 {
        fnv1a_hash(self.as_bytes())
    }
}

impl Hashable for &str {
    fn hash(&self) -> u64 {
        fnv1a_hash(self.as_bytes())
    }
}

impl Hashable for bool {
    fn hash(&self) -> u64 {
        if *self { 1 } else { 0 }
    }
}

// Implement Hashable for Vec
impl<T: Hashable> Hashable for Vec<T> {
    fn hash(&self) -> u64 {
        let mut hash = fnv1a_hash(&self.len().to_le_bytes());
        for item in self {
            // Combine hashes
            hash ^= item.hash();
            hash = hash.wrapping_mul(1099511628211);
        }
        hash
    }
}

// Implement Hashable for tuples
impl<T1: Hashable, T2: Hashable> Hashable for (T1, T2) {
    fn hash(&self) -> u64 {
        let h1 = self.0.hash();
        let h2 = self.1.hash();
        h1.wrapping_mul(31).wrapping_add(h2)
    }
}

impl<T1: Hashable, T2: Hashable, T3: Hashable> Hashable for (T1, T2, T3) {
    fn hash(&self) -> u64 {
        let h1 = self.0.hash();
        let h2 = self.1.hash();
        let h3 = self.2.hash();
        h1.wrapping_mul(31)
            .wrapping_add(h2)
            .wrapping_mul(31)
            .wrapping_add(h3)
    }
}

// Custom modulo that handles negative numbers properly
fn custom_mod(value: i64, modulus: i64) -> i64 {
    let result = value % modulus;
    if result < 0 {
        result + modulus
    } else {
        result
    }
}

// Hash and mod combined for hash table indexing
fn hash_and_mod<T: Hashable>(value: &T, table_size: usize) -> usize {
    let hash = value.hash();
    (hash % table_size as u64) as usize
}

// Complex data structure example
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    scores: Vec<i32>,
}

impl Hashable for Person {
    fn hash(&self) -> u64 {
        let name_hash = self.name.hash();
        let age_hash = self.age.hash();
        let scores_hash = self.scores.hash();
        
        // Combine all hashes
        name_hash
            .wrapping_mul(31)
            .wrapping_add(age_hash)
            .wrapping_mul(31)
            .wrapping_add(scores_hash)
    }
}

// Nested complex structure
#[derive(Debug)]
struct Company {
    name: String,
    employees: Vec<Person>,
    founded: i32,
}

impl Hashable for Company {
    fn hash(&self) -> u64 {
        let name_hash = self.name.hash();
        let employees_hash = self.employees.hash();
        let founded_hash = self.founded.hash();
        
        name_hash
            .wrapping_mul(31)
            .wrapping_add(employees_hash)
            .wrapping_mul(31)
            .wrapping_add(founded_hash)
    }
}

fn main() {
    println!("=== Basic Type Hashing ===");
    let num = 42;
    let text = "Hello, Rust!";
    let flag = true;
    
    println!("Hash of {}: {}", num, num.hash());
    println!("Hash of '{}': {}", text, text.hash());
    println!("Hash of {}: {}", flag, flag.hash());
    
    println!("\n=== Complex Type Hashing ===");
    let vec = vec![1, 2, 3, 4, 5];
    println!("Hash of {:?}: {}", vec, vec.hash());
    
    let tuple = ("Alice", 30);
    println!("Hash of {:?}: {}", tuple, tuple.hash());
    
    println!("\n=== Custom Struct Hashing ===");
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
        scores: vec![85, 90, 78],
    };
    println!("Hash of {:?}: {}", person, person.hash());
    
    let company = Company {
        name: "TechCorp".to_string(),
        employees: vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
                scores: vec![95, 88],
            },
            Person {
                name: "Charlie".to_string(),
                age: 28,
                scores: vec![82, 90, 85],
            },
        ],
        founded: 2010,
    };
    println!("Hash of company: {}", company.hash());
    
    println!("\n=== Modulo Operations ===");
    let table_size = 10;
    println!("Hash of 'Alice' mod {}: {}", table_size, hash_and_mod(&"Alice", table_size));
    println!("Hash of 'Bob' mod {}: {}", table_size, hash_and_mod(&"Bob", table_size));
    println!("Hash of 42 mod {}: {}", table_size, hash_and_mod(&42, table_size));
    
    println!("\n=== Custom Mod (handles negatives) ===");
    println!("custom_mod(-15, 7) = {}", custom_mod(-15, 7));
    println!("custom_mod(15, 7) = {}", custom_mod(15, 7));
    println!("custom_mod(-1, 10) = {}", custom_mod(-1, 10));
    
    println!("\n=== Hash Distribution Test ===");
    let names = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Henry"];
    println!("Distributing {} names into {} buckets:", names.len(), table_size);
    for name in &names {
        let bucket = hash_and_mod(name, table_size);
        println!("  {} -> bucket {}", name, bucket);
    }
    
    println!("\n=== Different Hash Algorithms ===");
    let data = "test data";
    println!("FNV-1a hash: {}", fnv1a_hash(data.as_bytes()));
    println!("DJB2 hash: {}", djb2_hash(data.as_bytes()));
}
