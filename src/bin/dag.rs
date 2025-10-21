use std::collections::{HashMap, HashSet, VecDeque};

/// 有向無環圖 (Directed Acyclic Graph)
#[derive(Debug, Clone)]
pub struct DAG<T: Clone + Eq + std::hash::Hash> {
    // 使用 adjacency list 表示圖
    // key: 節點, value: 該節點指向的所有節點
    adj_list: HashMap<T, Vec<T>>,
}

impl<T: Clone + Eq + std::hash::Hash> DAG<T> {
    /// 建立新的空 DAG
    pub fn new() -> Self {
        DAG {
            adj_list: HashMap::new(),
        }
    }

    /// 新增節點
    pub fn add_node(&mut self, node: T) {
        self.adj_list.entry(node).or_insert_with(Vec::new);
    }

    /// 新增邊 (from -> to)
    /// 如果會造成循環，返回 Err
    pub fn add_edge(&mut self, from: T, to: T) -> Result<(), String> {
        // 先檢查是否會造成循環
        if self.would_create_cycle(&from, &to) {
            return Err(format!("Adding edge would create a cycle"));
        }

        self.add_node(from.clone());
        self.add_node(to.clone());
        
        if let Some(neighbors) = self.adj_list.get_mut(&from) {
            if !neighbors.contains(&to) {
                neighbors.push(to);
            }
        }
        
        Ok(())
    }

    /// 檢查新增邊是否會造成循環
    fn would_create_cycle(&self, from: &T, to: &T) -> bool {
        // 如果 to 能到達 from，那麼新增 from->to 會造成循環
        self.can_reach(to, from)
    }

    /// 檢查是否能從 start 到達 target (使用 BFS)
    fn can_reach(&self, start: &T, target: &T) -> bool {
        if start == target {
            return true;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start.clone());
        visited.insert(start.clone());

        while let Some(node) = queue.pop_front() {
            if let Some(neighbors) = self.adj_list.get(&node) {
                for neighbor in neighbors {
                    if neighbor == target {
                        return true;
                    }
                    if visited.insert(neighbor.clone()) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        false
    }

    /// 拓撲排序 (Topological Sort) - 使用 Kahn's Algorithm
    pub fn topological_sort(&self) -> Result<Vec<T>, String> {
        // 計算每個節點的入度 (in-degree)
        let mut in_degree: HashMap<T, usize> = HashMap::new();
        
        for node in self.adj_list.keys() {
            in_degree.entry(node.clone()).or_insert(0);
        }
        
        for neighbors in self.adj_list.values() {
            for neighbor in neighbors {
                *in_degree.entry(neighbor.clone()).or_insert(0) += 1;
            }
        }

        // 找出所有入度為 0 的節點
        let mut queue: VecDeque<T> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(node, _)| node.clone())
            .collect();

        let mut result = Vec::new();

        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            if let Some(neighbors) = self.adj_list.get(&node) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        // 如果結果數量不等於節點數量，代表有循環
        if result.len() != self.adj_list.len() {
            return Err("Graph contains a cycle".to_string());
        }

        Ok(result)
    }

    /// 深度優先搜索 (DFS)
    pub fn dfs(&self, start: &T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_helper(start, &mut visited, &mut result);
        result
    }

    fn dfs_helper(&self, node: &T, visited: &mut HashSet<T>, result: &mut Vec<T>) {
        if visited.contains(node) {
            return;
        }

        visited.insert(node.clone());
        result.push(node.clone());

        if let Some(neighbors) = self.adj_list.get(node) {
            for neighbor in neighbors {
                self.dfs_helper(neighbor, visited, result);
            }
        }
    }

    /// 取得所有節點
    pub fn nodes(&self) -> Vec<T> {
        self.adj_list.keys().cloned().collect()
    }

    /// 取得節點的鄰居
    pub fn neighbors(&self, node: &T) -> Option<&Vec<T>> {
        self.adj_list.get(node)
    }
}

// 範例使用
fn main() {
    println!("=== DAG 範例 1: 任務依賴 ===\n");
    
    let mut dag = DAG::new();
    
    // 建立任務依賴圖
    // A -> B (A 必須在 B 之前完成)
    // A -> C
    // B -> D
    // C -> D
    dag.add_edge("A", "B").unwrap();
    dag.add_edge("A", "C").unwrap();
    dag.add_edge("B", "D").unwrap();
    dag.add_edge("C", "D").unwrap();

    println!("拓撲排序 (執行順序):");
    if let Ok(order) = dag.topological_sort() {
        println!("{:?}", order);
    }

    println!("\n從 A 開始的 DFS:");
    println!("{:?}", dag.dfs(&"A"));

    println!("\n=== DAG 範例 2: 課程先修條件 ===\n");
    
    let mut courses = DAG::new();
    courses.add_edge("數學101", "數學201").unwrap();
    courses.add_edge("數學201", "數學301").unwrap();
    courses.add_edge("程式設計", "資料結構").unwrap();
    courses.add_edge("資料結構", "演算法").unwrap();
    courses.add_edge("數學101", "演算法").unwrap();

    println!("課程修習順序:");
    if let Ok(order) = courses.topological_sort() {
        for (i, course) in order.iter().enumerate() {
            println!("{}. {}", i + 1, course);
        }
    }

    println!("\n=== DAG 範例 3: 檢測循環 ===\n");
    
    let mut cycle_test = DAG::new();
    cycle_test.add_edge(1, 2).unwrap();
    cycle_test.add_edge(2, 3).unwrap();
    
    // 嘗試新增會造成循環的邊
    match cycle_test.add_edge(3, 1) {
        Ok(_) => println!("成功新增邊 3->1"),
        Err(e) => println!("無法新增邊 3->1: {}", e),
    }

    println!("\n當前圖的拓撲排序: {:?}", cycle_test.topological_sort().unwrap());
}

/*
=== DAG 範例 1: 任務依賴 ===

拓撲排序 (執行順序):
["A", "B", "C", "D"]

從 A 開始的 DFS:
["A", "B", "D", "C"]

=== DAG 範例 2: 課程先修條件 ===

課程修習順序:
1. 數學101
2. 程式設計
3. 數學201
4. 資料結構
5. 數學301
6. 演算法

=== DAG 範例 3: 檢測循環 ===

無法新增邊 3->1: Adding edge would create a cycle

當前圖的拓撲排序: [1, 2, 3]
/*
