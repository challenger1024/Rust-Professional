use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::Value;
use std::path::Path;

// 并查集数据结构
struct UnionFind {
    parent: HashMap<String, String>,
}

impl UnionFind {
    // 初始化并查集
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
        }
    }

    // 查找根节点
    fn find(&mut self, x: &str) -> String {
        let p = self.parent.get(x).cloned();
        
        if let Some(p) = p {
            if p != x {
                let root = self.find(&p);
                self.parent.insert(x.to_string(), root.clone());
                return root;
            }
        } else {
            self.parent.insert(x.to_string(), x.to_string());
        }
        
        x.to_string()
    }

    // 合并两个集合
    fn union(&mut self, x: &str, y: &str) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent.insert(root_y, root_x);
        }
    }
}

// 计算省份数量
pub fn count_provinces() -> String {
    let file_path = Path::new("district.json");
    if !file_path.exists() {
        panic!("district.json file not found in the parent directory");
    }

    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let json: Value = serde_json::from_str(&data).expect("Failed to parse JSON");

    let mut results = Vec::new();

    for (_, batch) in json.as_object().unwrap() {
        let mut uf = UnionFind::new();
        let mut cities = HashSet::new();

        for (city, neighbors) in batch.as_object().unwrap() {
            cities.insert(city.clone());
            for neighbor in neighbors.as_array().unwrap() {
                let neighbor = neighbor.as_str().unwrap();
                cities.insert(neighbor.to_string());
                uf.union(city, neighbor);
            }
        }

        let mut provinces = HashSet::new();
        for city in &cities {  // 这里改为 `&cities`
            let root = uf.find(city);  // `city` 现在是 `&String`
            provinces.insert(root);
        }
        results.push(provinces.len().to_string());
    }

    results.join(",")
}
