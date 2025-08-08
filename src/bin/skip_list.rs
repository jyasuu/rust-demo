use std::cmp::Ordering;
use std::fmt::Debug;
use std::sync::atomic::{AtomicU32, Ordering as AtomicOrdering};

const MAX_LEVEL: usize = 16;

// Global counter for seed generation
static SEED_COUNTER: AtomicU32 = AtomicU32::new(1);

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    forward: Vec<Option<usize>>, // indices to next nodes at each level
}

impl<T> Node<T> {
    fn new(value: T, level: usize) -> Self {
        Node {
            value,
            forward: vec![None; level + 1],
        }
    }
}

#[derive(Debug)]
pub struct SkipList<T> {
    nodes: Vec<Node<T>>,
    head_forward: Vec<Option<usize>>, // head's forward pointers
    level: usize,
    rng_state: u32, // Simple LCG for random number generation
}

impl<T: Ord + Clone + Debug> SkipList<T> {
    pub fn new() -> Self {
        // Generate a better seed using global counter and stack address
        let counter = SEED_COUNTER.fetch_add(1, AtomicOrdering::Relaxed);
        let stack_addr = &counter as *const u32 as usize;
        let seed = (counter.wrapping_mul(31) ^ (stack_addr as u32)) | 1; // Ensure odd
        
        SkipList {
            nodes: Vec::new(),
            head_forward: vec![None; MAX_LEVEL + 1],
            level: 0,
            rng_state: seed,
        }
    }

    // Xorshift random number generator (better than LCG)
    fn next_random(&mut self) -> u32 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 17;
        self.rng_state ^= self.rng_state << 5;
        self.rng_state
    }

    fn random_level(&mut self) -> usize {
        let mut level = 0;
        // Keep generating higher levels with 50% probability
        while level < MAX_LEVEL {
            let rand_val = self.next_random();
            println!("    🎲 Random value: {}, mod 2: {}", rand_val, rand_val % 2);
            if (rand_val % 2) == 0 {
                level += 1;
            } else {
                break;
            }
        }
        level
    }

    fn get_forward(&self, node_idx: Option<usize>, level: usize) -> Option<usize> {
        match node_idx {
            None => self.head_forward.get(level).copied().flatten(),
            Some(idx) => {
                if idx < self.nodes.len() && level < self.nodes[idx].forward.len() {
                    self.nodes[idx].forward[level]
                } else {
                    None
                }
            }
        }
    }

    fn set_forward(&mut self, node_idx: Option<usize>, level: usize, target_idx: Option<usize>) {
        match node_idx {
            None => {
                if level < self.head_forward.len() {
                    self.head_forward[level] = target_idx;
                }
            }
            Some(idx) => {
                if idx < self.nodes.len() && level < self.nodes[idx].forward.len() {
                    self.nodes[idx].forward[level] = target_idx;
                }
            }
        }
    }

    pub fn insert(&mut self, value: T) {
        println!("\n🔍 Inserting: {:?}", value);
        
        let mut update = vec![None; MAX_LEVEL + 1];
        let mut current_idx = None; // None represents head

        // Find insertion point
        for level in (0..=self.level).rev() {
            while let Some(next_idx) = self.get_forward(current_idx, level) {
                if next_idx < self.nodes.len() {
                    match self.nodes[next_idx].value.cmp(&value) {
                        Ordering::Less => current_idx = Some(next_idx),
                        _ => break,
                    }
                } else {
                    break;
                }
            }
            update[level] = current_idx;
        }

        // Check if value already exists
        if let Some(next_idx) = self.get_forward(current_idx, 0) {
            if next_idx < self.nodes.len() && self.nodes[next_idx].value == value {
                println!("❌ Value {:?} already exists, skipping insertion", value);
                self.display();
                return;
            }
        }

        let new_level = self.random_level();
        println!("📊 Assigned level: {}", new_level);
        
        if new_level > self.level {
            for level in (self.level + 1)..=new_level {
                update[level] = None; // Point to head
            }
            self.level = new_level;
            println!("📈 Skip list level increased to: {}", self.level);
        }

        // Create new node
        let new_idx = self.nodes.len();
        let mut new_node = Node::new(value.clone(), new_level);
        
        // Set up forward pointers for new node
        for level in 0..=new_level {
            new_node.forward[level] = self.get_forward(update[level], level);
            self.set_forward(update[level], level, Some(new_idx));
        }

        self.nodes.push(new_node);
        println!("✅ Successfully inserted {:?} at index {}", value, new_idx);
        self.display();
    }

    pub fn search(&self, value: &T) -> bool {
        println!("\n🔍 Searching for: {:?}", value);
        
        let mut current_idx = None; // Start at head
        let mut path = Vec::new(); // Track search path

        for level in (0..=self.level).rev() {
            // Track search at this level
            while let Some(next_idx) = self.get_forward(current_idx, level) {
                if next_idx < self.nodes.len() {
                    match self.nodes[next_idx].value.cmp(value) {
                        Ordering::Less => {
                            current_idx = Some(next_idx);
                            path.push((level, next_idx, self.nodes[next_idx].value.clone()));
                        }
                        Ordering::Equal => {
                            path.push((level, next_idx, self.nodes[next_idx].value.clone()));
                            println!("🔍 Search path: {:?}", path);
                            println!("✅ Found {:?} at index {}", value, next_idx);
                            return true;
                        }
                        Ordering::Greater => break,
                    }
                } else {
                    break;
                }
            }
        }
        
        println!("🔍 Search path: {:?}", path);
        println!("❌ Value {:?} not found", value);
        false
    }

    pub fn delete(&mut self, value: &T) -> bool {
        println!("\n🗑️ Deleting: {:?}", value);
        
        let mut update = vec![None; MAX_LEVEL + 1];
        let mut current_idx = None;

        // Find the node to delete
        for level in (0..=self.level).rev() {
            while let Some(next_idx) = self.get_forward(current_idx, level) {
                if next_idx < self.nodes.len() {
                    match self.nodes[next_idx].value.cmp(value) {
                        Ordering::Less => current_idx = Some(next_idx),
                        _ => break,
                    }
                } else {
                    break;
                }
            }
            update[level] = current_idx;
        }

        // Check if the node exists
        if let Some(target_idx) = self.get_forward(current_idx, 0) {
            if target_idx < self.nodes.len() && self.nodes[target_idx].value == *value {
                println!("📍 Found node to delete at index {}", target_idx);
                
                // Update forward pointers
                let target_level = self.nodes[target_idx].forward.len() - 1;
                for level in 0..=target_level {
                    if level <= self.level {
                        let next_target = self.nodes[target_idx].forward[level];
                        self.set_forward(update[level], level, next_target);
                    }
                }

                // Remove the node (this invalidates indices, so we'll mark it as deleted instead)
                // In a real implementation, you might use a different approach
                self.nodes.remove(target_idx);
                
                // Update all forward pointers that point beyond the removed index
                for level in 0..=self.level {
                    if let Some(ref mut forward_idx) = self.head_forward[level] {
                        if *forward_idx > target_idx {
                            *forward_idx -= 1;
                        } else if *forward_idx == target_idx {
                            // This should have been handled above, but just in case
                            self.head_forward[level] = None;
                        }
                    }
                }
                
                for node in &mut self.nodes {
                    for forward_ref in &mut node.forward {
                        if let Some(ref mut forward_idx) = forward_ref {
                            if *forward_idx > target_idx {
                                *forward_idx -= 1;
                            }
                        }
                    }
                }

                // Update skip list level if necessary
                while self.level > 0 && self.head_forward[self.level].is_none() {
                    self.level -= 1;
                }

                println!("✅ Successfully deleted {:?}", value);
                self.display();
                return true;
            }
        }
        
        println!("❌ Value {:?} not found, cannot delete", value);
        self.display();
        false
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn display(&self) {
        println!("\n📋 Skip List Structure (Current Level: {}, Nodes: {})", self.level, self.len());
        println!("{}", "═".repeat(60));
        
        if self.is_empty() {
            println!("Empty skip list");
            println!("{}", "═".repeat(60));
            return;
        }

        for level in (0..=self.level).rev() {
            print!("Level {:2} │ HEAD", level);
            let mut current_idx = None;
            let mut position = 0;
            
            // Track all nodes at level 0 for proper spacing
            let mut all_nodes = Vec::new();
            let mut temp_idx = self.head_forward[0];
            while let Some(idx) = temp_idx {
                if idx < self.nodes.len() {
                    all_nodes.push((idx, &self.nodes[idx].value));
                    temp_idx = self.nodes[idx].forward[0];
                } else {
                    break;
                }
            }
            
            // Show nodes at this level with proper spacing
            while let Some(next_idx) = self.get_forward(current_idx, level) {
                if next_idx < self.nodes.len() {
                    // Find position of this node in the overall sequence
                    while position < all_nodes.len() && all_nodes[position].0 != next_idx {
                        print!(" ─ ─ ─ ");
                        position += 1;
                    }
                    if position < all_nodes.len() {
                        print!(" → {:?}", self.nodes[next_idx].value);
                        current_idx = Some(next_idx);
                        position += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            
            // Fill remaining positions with dashes
            while position < all_nodes.len() {
                print!(" ─ ─ ─ ");
                position += 1;
            }
            
            println!(" → NIL");
        }
        println!("{}", "═".repeat(60));
    }

    pub fn display_detailed(&self) {
        println!("\n🔍 Detailed Skip List Analysis");
        println!("{}", "═".repeat(80));
        println!("Total nodes: {}", self.len());
        println!("Current max level: {}", self.level);
        
        if self.is_empty() {
            println!("List is empty");
            return;
        }

        println!("\nNode Details:");
        for (idx, node) in self.nodes.iter().enumerate() {
            println!("Node {}: Value={:?}, Level={}, Forward={:?}", 
                     idx, node.value, node.forward.len() - 1, node.forward);
        }
        
        println!("\nHead Forward Pointers: {:?}", self.head_forward);
        println!("{}", "═".repeat(80));
    }
}

impl<T: Ord + Clone + Debug> Default for SkipList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_list_operations() {
        let mut list = SkipList::new();
        
        // Test insertions
        list.insert(3);
        list.insert(6);
        list.insert(7);
        list.insert(9);
        list.insert(12);
        
        // Test search
        assert!(list.search(&6));
        assert!(list.search(&9));
        assert!(!list.search(&5));
        
        // Test deletion
        assert!(list.delete(&6));
        assert!(!list.search(&6));
        assert!(!list.delete(&100)); // Non-existent value
        
        assert_eq!(list.len(), 4);
    }
}

fn main() {
    println!("🚀 Skip List Demo with Standard Library Only");
    println!("{}", "═".repeat(60));
    
    let mut list = SkipList::new();
    
    println!("📝 Starting with empty skip list:");
    list.display();
    
    // Insert operations
    let values = vec![3, 6, 7, 9, 12, 19, 17, 26, 21, 25];
    println!("\n🔄 Inserting values: {:?}", values);
    
    for value in values {
        list.insert(value);
    }
    
    for i in 0..100 {
        list.insert(i * 100);
        
    }
    
    // Try to insert duplicate
    println!("\n🔄 Attempting to insert duplicate value:");
    list.insert(12);
    
    // Search operations
    println!("\n🔄 Search operations:");
    let search_values = vec![6, 19, 5, 30, 21];
    for value in search_values {
        list.search(&value);
    }
    
    // Delete operations
    println!("\n🔄 Delete operations:");
    let delete_values = vec![19, 5, 12];
    for value in delete_values {
        list.delete(&value);
    }
    
    // Final state
    println!("\n🏁 Final skip list state:");
    list.display_detailed();
}
