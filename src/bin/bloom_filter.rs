use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// 布隆過濾器結構
pub struct BloomFilter<T: ?Sized> {
    bitmap: Vec<u8>,
    size: usize,
    hash_count: usize,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized + Hash> BloomFilter<T> {
    /// 創建新的布隆過濾器
    /// - expected_items: 預期存儲的元素數量
    /// - false_positive_rate: 期望的誤判率
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        assert!(false_positive_rate > 0.0 && false_positive_rate < 1.0);
        
        // 計算最優位數組大小
        let size = Self::optimal_size(expected_items, false_positive_rate);
        
        // 計算最優哈希函數數量
        let hash_count = Self::optimal_hash_count(size, expected_items);
        
        BloomFilter {
            bitmap: vec![0; (size + 7) / 8], // 位數組 (按位存儲)
            size,
            hash_count,
            _phantom: PhantomData,
        }
    }
    
    /// 添加元素到過濾器
    pub fn insert(&mut self, item: &T) {
        for i in 0..self.hash_count {
            let index = self.get_hash(item, i) % self.size;
            self.set_bit(index);
        }
    }
    
    /// 檢查元素是否存在
    pub fn contains(&self, item: &T) -> bool {
        for i in 0..self.hash_count {
            let index = self.get_hash(item, i) % self.size;
            if !self.get_bit(index) {
                return false;
            }
        }
        true
    }
    
    // 計算最優位數組大小
    fn optimal_size(expected_items: usize, false_positive_rate: f64) -> usize {
        let ln2_2 = std::f64::consts::LN_2 * std::f64::consts::LN_2;
        ((-1.0f64 * expected_items as f64 * false_positive_rate.ln()) / ln2_2).ceil() as usize
    }
    
    // 計算最優哈希函數數量
    fn optimal_hash_count(size: usize, expected_items: usize) -> usize {
        ((size as f64 / expected_items as f64) * std::f64::consts::LN_2).ceil() as usize
    }
    
    // 獲取元素的哈希值
    fn get_hash(&self, item: &T, seed: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        item.hash(&mut hasher);
        hasher.finish() as usize
    }
    
    // 設置位數組中的位
    fn set_bit(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.bitmap[byte_index] |= 1 << bit_index;
    }
    
    // 獲取位數組中的位
    fn get_bit(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_index = index % 8;
        (self.bitmap[byte_index] & (1 << bit_index)) != 0
    }
}

/// 真實業務場景示例：惡意URL檢測系統
pub struct MaliciousUrlChecker {
    bloom_filter: BloomFilter<String>,
}

impl MaliciousUrlChecker {
    pub fn new(expected_urls: usize, false_positive_rate: f64) -> Self {
        Self {
            bloom_filter: BloomFilter::new(expected_urls, false_positive_rate),
        }
    }
    
    /// 添加惡意URL到數據庫
    pub fn add_malicious_url(&mut self, url: &str) {
        self.bloom_filter.insert(&url.to_string());
    }
    
    /// 檢查URL是否惡意
    pub fn is_malicious(&self, url: &str) -> bool {
        self.bloom_filter.contains(&url.to_string())
    }
}

fn main() {
    // 創建惡意URL檢測系統
    // 預期存儲10萬個URL，誤判率0.1%
    let mut checker = MaliciousUrlChecker::new(100_000, 0.001);
    
    // 添加惡意URL
    checker.add_malicious_url("https://phishing-site.com");
    checker.add_malicious_url("https://malware-download.com");
    checker.add_malicious_url("https://scam-page.org");
    
    // 測試URL檢測
    println!("檢查合法URL: {}", checker.is_malicious("https://safe-website.com")); // 應該為false
    println!("檢查惡意URL: {}", checker.is_malicious("https://phishing-site.com")); // 應該為true
    
    // 測試誤判率
    let test_url = "https://legitimate-site-";
    let mut false_positives = 0;
    let total_tests = 10_000;
    
    for i in 0..total_tests {
        if checker.is_malicious(&format!("{}{}", test_url, i)) {
            false_positives += 1;
        }
    }
    
    println!(
        "實際誤判率: {:.4}%",
        (false_positives as f64 / total_tests as f64) * 100.0
    );
}


// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1e4e7fd4f2673483821e8eab9528f38b
