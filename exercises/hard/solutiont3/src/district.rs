use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::Value;
use std::path::Path;

// 并查集数据结构
struct UnionFind {
    parent: HashMap<String, String>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
        }
    }

    fn find(&mut self, x: &str) -> String {
        if !self.parent.contains_key(x) {
            self.parent.insert(x.to_string(), x.to_string());
        }
        let parent = self.parent.get(x).unwrap().clone();
        if parent != x {
            let root = self.find(&parent);
            self.parent.insert(x.to_string(), root.clone()); // 路径压缩
            return root;
        }
        x.to_string()
    }

    fn union(&mut self, x: &str, y: &str) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent.insert(root_y, root_x);
        }
    }
}

pub fn count_provinces() -> String {
    let file_path = Path::new("district.json");
    if !file_path.exists() {
        panic!("district.json file not found in the project root directory");
    }

    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let json: Value = serde_json::from_str(&data).expect("Failed to parse JSON");

    let mut results = Vec::new();
    let mut keys: Vec<&String> = json.as_object().unwrap().keys().collect();
    keys.sort_by_key(|k| k.parse::<i32>().unwrap());

    for key in keys {
        let batch = &json[key];
        let mut uf = UnionFind::new();
        let mut cities = HashSet::new();
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

        // **收集所有城市及其邻接城市**
        let mut edges = Vec::new();
        for (city, neighbors) in batch.as_object().unwrap() {
            for neighbor in neighbors.as_array().unwrap() {
                if let Some(neighbor) = neighbor.as_str() {
                    if city != neighbor { // **忽略自连接**
                        edges.push((city.clone(), neighbor.to_string()));
                    }
                }
            }
        }

        // **构建邻接表**
        for (city, neighbor) in edges {
            graph.entry(city.clone()).or_insert_with(HashSet::new).insert(neighbor.clone());
            graph.entry(neighbor).or_insert_with(HashSet::new).insert(city.clone());
        }

        // **构建并查集**
        for (city, neighbors) in &graph {
            cities.insert(city.clone());
            for neighbor in neighbors {
                uf.union(city, neighbor);
            }
        }

        // **计算省份数**
        let mut provinces = HashSet::new();
        for city in &cities {
            let root = uf.find(city);
            provinces.insert(root);
        }
        results.push(provinces.len().to_string());
    }

    results.join(",")
}
