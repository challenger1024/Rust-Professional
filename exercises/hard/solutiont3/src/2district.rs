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

    // 查找根节点（路径压缩优化）
    fn find(&mut self, x: &str) -> String {
        // 确保 x 在 parent 中
        if !self.parent.contains_key(x) {
            self.parent.insert(x.to_string(), x.to_string());
        }

        // 获取 x 的父节点
        let parent = self.parent.get(x).unwrap().clone();
        if parent != x {
            let root = self.find(&parent);
            self.parent.insert(x.to_string(), root.clone()); // 路径压缩
            return root;
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
        panic!("district.json file not found in the project root directory");
    }

    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let json: Value = serde_json::from_str(&data).expect("Failed to parse JSON");

    let mut results = Vec::new();

    // 获取批次 key，并按照数值排序
    let mut keys: Vec<&String> = json.as_object().unwrap().keys().collect();
    keys.sort_by_key(|k| k.parse::<i32>().unwrap()); // 按批次号升序排序

    for key in keys {
        let batch = &json[key]; // 按排序顺序取出批次数据
        let mut uf = UnionFind::new();
        let mut cities = HashSet::new();

        for (city, neighbors) in batch.as_object().unwrap() {
            cities.insert(city.to_string());
            for neighbor in neighbors.as_array().unwrap() {
                if let Some(neighbor) = neighbor.as_str() {
                    cities.insert(neighbor.to_string());
                    uf.union(city, neighbor);
                }
            }
        }

        let mut provinces = HashSet::new();
        for city in &cities {
            let root = uf.find(city);
            provinces.insert(root);
        }
        results.push(provinces.len().to_string());
    }

    results.join(",")
}
