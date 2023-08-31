// fn scale(q: f32, delta: f32) -> f32 {
//     if q >= 0.5 {
//         delta - delta * (2.0 - 2.0 * q).sqrt() / 2.0
//     } else {
//         delta * (2.0 * q).sqrt() / 2.0
//     }
// }

// inverse of the scale function (commented above)
fn inverse_scale(k: f32, delta: f32) -> f32 {
    let kd_ratio = k / delta; // funny gamer joke. see the reason it's funny is because
    if kd_ratio >= 0.5 {
        1.0 - 2.0 * f32::powi(1.0 - kd_ratio, 2)
    } else {
        2.0 * f32::powi(kd_ratio, 2)
    }
}

#[derive(Clone, Copy)]
struct Centroid {
    mean: f32,
    weight: i32,
}

impl Centroid {
    // combines two centroids. probably room for optimization here
    fn merge(&mut self, c: &Centroid) {
        let sum_weights = self.weight + c.weight;
        let w1 = self.weight as f32 / sum_weights as f32;
        let w2 = c.weight as f32 / sum_weights as f32;

        let mean = w1 * self.mean + w2 * c.mean;
        let weight = self.weight + c.weight;

        self.mean = mean;
        self.weight = weight;
    }
}

/// T-Digest algorithm. A probabilistic data structure for estimating quantiles of data while using a fixed amount of memory.
pub struct TDigest {
    centroids: Vec<Centroid>,
    delta: f32,
    temp_centroids: Vec<Centroid>, // used exclusively for cluster_centroids to save on vector allocations
}

impl TDigest {
    /// Creates a new empty T-Digest given a delta value.
    pub fn new_empty(delta: f32) -> Self {
        Self {
            centroids: Vec::new(),
            delta,
            temp_centroids: Vec::new(),
        }
    }

    // the following is not necessary for the python package (you can only have one new function)
    /// Creates a new T-Digest from an input vector of unsorted values and a delta value. Equivalent to creating an empty T-Digest and merging in a vector of values.
    // pub fn new_from_vec(v: Vec<f32>, delta: f32) -> Self {
    //     let mut digest = Self {
    //         centroids: Vec::new(),
    //         delta,
    //         buffer: [0.0; 32],
    //         index: 0,
    //         temp_centroids: Vec::new(),
    //     };
    //     digest.merge_vec_unsorted(v);
    //     digest
    // }

    fn vals_to_centroids(v: impl IntoIterator<Item = f32>) -> impl IntoIterator<Item = Centroid> {
        v.into_iter().map(|x| Centroid {mean: x, weight: 1})
    }

    // THIS FUNCTION WILL DIE IF THE INTERNAL CENTROIDS AREN'T SORTED BY MEAN!!!!!
    // takes an unoptimized list of centroids and condenses it down to fit within the digest's delta value
    fn cluster_centroids(&mut self) {
        if self.centroids.len() == 0 {
            return
        }
        self.temp_centroids.clear(); // used as a buffer to store the new centroids while accessing the old
        let weight_sum: i32 = self.centroids.iter().map(|x| x.weight).sum(); // sum of weights of all centroids (equivalent to total number of values added to digest)
        let mut k_limit: f32 = 1.0; // if you read the gresearch article, it's actually slightly wrong. the new group of centroids doesn't start at the one that pushes the delta k past 1, it starts at each k whole number (so that you don't end up with >delta centroids)
        let mut q_lower: f32 = 0.0; // lower quantile limit. feel like there's a small optimization to be had here but i'm not seeing it.
        let mut q_limit = inverse_scale(k_limit, self.delta); // precomputed as a max quantile value here & after each new centroid is completed so that you don't have to compute new k values with every single iteration
        let mut current_centroid = self.centroids[0]; // also used as a buffer, stores the aggregated centroid that is to be added to temp_centroids
        for i in 1..self.centroids.len() {
            let q_upper = q_lower + (current_centroid.weight + self.centroids[i].weight) as f32 / weight_sum as f32; // quantiles are computed as aggregated weight / total weight
            if q_upper <= q_limit {
                current_centroid.merge(&self.centroids[i]); // add another to the beast. reminds me of the meatball man
            } else {
                self.temp_centroids.push(current_centroid);
                q_lower += current_centroid.weight as f32 / weight_sum as f32;
                k_limit += 1.0;
                q_limit = inverse_scale(k_limit, self.delta);
                current_centroid = self.centroids[i];
            }
        }
        self.temp_centroids.push(current_centroid);
        std::mem::swap(&mut self.temp_centroids, &mut self.centroids);
    }

    // binary search to find the centroid with the next largest mean after given mean
    // will typically use O(log n) time, could take less in the case of an exact match
    // not sure how much optimization there is to be had here aside from a better algorithm
    fn search_centroids(&self, mean: f32) -> usize {
        if self.centroids.len() == 0 {
            return 0;
        }

        let mut lower: usize = 0;
        let mut upper: usize = self.centroids.len() - 1;
        if mean < self.centroids[lower].mean {
            return 0;
        } else if mean > self.centroids[upper].mean {
            return upper + 1;
        }
        loop {
            let center = ((lower + upper) as f32 / 2.0).floor() as usize;
            if mean > self.centroids[center].mean && mean <= self.centroids[center + 1].mean {
                return center + 1;
            } else if mean == self.centroids[center].mean {
                return center;
            }
            let dist = |index: &usize| (self.centroids[*index].mean - mean).abs();
            let l = dist(&center);
            let r = dist(&(center + 1));
            if l > r {
                lower = center + 1;
            } else {
                upper = center;
            }
        }
    }

    // not sure if this function could be optimized, but its usage can
    // gets the weight-based quantile of a centroid in the digest (given by index)
    fn centroid_quantile(&self, index: usize) -> f32 {
        let weight_to_index = self.centroids.iter().take(index + 1).map(|c| c.weight).sum::<i32>() as f32;
        let total_weight = weight_to_index + self.centroids.iter().skip(index + 1).map(|c| c.weight).sum::<i32>() as f32;
        (weight_to_index - (self.centroids[index].weight as f32 / 2.0)) / total_weight
    }

    // TODO: rewrite using IntoIter instead of vec
    fn merge_sorted_centroids(&mut self, b: Vec<Centroid>) {
        let mut j = 0;
        let mut i = 0;
        if b.len() == 0 {
            return
        }
        while i < self.centroids.len() {
            if b[j].mean <= self.centroids[i].mean {
                self.centroids.insert(i, b[j]);
                j += 1;
            }
            i += 1;
        }
        self.centroids.extend(b.iter().skip(j));
    }

    // based on algorithm in gresearch paper
    /// Estimates what quantile a given value is in.
    pub fn estimate_quantile(&self, v: f32) -> f32 {
        let index = self.search_centroids(v);
        if index == 0 {
            return 0.0;
        } else if index == self.centroids.len() {
            return 1.0;
        }
        let quantile_l = self.centroid_quantile(index - 1);
        let quantile_r = self.centroid_quantile(index); // this is basically free optimization but i was too lazy to make my code messier. just add the quantile of the centroid at index instead of recomputing everything here
        let lerp_val = (v - self.centroids[index - 1].mean) / (self.centroids[index].mean - self.centroids[index - 1].mean);
        quantile_l + lerp_val * (quantile_r - quantile_l)
    }

    // shamefully stolen from the better tdigest rust crate
    /// Estimates the value at the given quantile.
    pub fn estimate_value(&self, q: f32) -> f32 {
        if self.centroids.len() == 0 {
            return 0.0;
        }

        let count_ = self.centroids.iter().map(|c| c.weight).sum::<i32>() as f32;
        let rank = count_ * q;

        let self_min = self.centroids[0].mean;
        let self_max = self.centroids.last().expect("uh oh").mean;

        // the following chunk of code does the same thing as search_centroids but it's based on quantile instead of mean and also it's much faster for the use case (most queries are going to be at the extremes, so linear search works out to be pretty fast)
        let mut pos: usize;
        let mut t: f32;
        match q {
            q if q <= 0.0 => return self_min,
            q if q >= 1.0 => return self_max,
            q if q > 0.5 => {
                pos = 0;
                t = count_;
    
                for (k, centroid) in self.centroids.iter().enumerate().rev() {
                    t -= centroid.weight as f32;
    
                    if rank >= t {
                        pos = k;
                        break;
                    }
                }
            },
            _ => {
                pos = self.centroids.len() - 1;
                t = 0.0;
    
                for (k, centroid) in self.centroids.iter().enumerate() {
                    if rank < t + centroid.weight as f32 {
                        pos = k;
                        break;
                    }
    
                    t += centroid.weight as f32;
                }
            }
        }

        // TODO: document this lmao
        let mut delta = 0.0;
        let mut min: f32 = self_min.clone();
        let mut max: f32 = self_max.clone();

        if self.centroids.len() > 1 {
            if pos == 0 {
                delta = self.centroids[pos + 1].mean - self.centroids[pos].mean;
                max = self.centroids[pos + 1].mean;
            } else if pos == (self.centroids.len() - 1) {
                delta = self.centroids[pos].mean - self.centroids[pos - 1].mean;
                min = self.centroids[pos - 1].mean;
            } else {
                delta = (self.centroids[pos + 1].mean - self.centroids[pos - 1].mean) / 2.0;
                min = self.centroids[pos - 1].mean;
                max = self.centroids[pos + 1].mean;
            }
        }

        let value = self.centroids[pos].mean + ((rank - t) / self.centroids[pos].weight as f32 - 0.5) * delta;
        value.max(min).min(max)
    }
    
    /// Merges an unsorted vector of values into the T-Digest.
    pub fn merge_vec_unsorted(&mut self, v: impl IntoIterator<Item = f32>) {
        self.centroids.extend(Self::vals_to_centroids(v));
        self.centroids.sort_unstable_by(|a, b| a.mean.partial_cmp(&b.mean).unwrap());
        self.cluster_centroids();
    }

    /// Merges a sorted vector of values into the T-Digest.
    pub fn merge_vec_sorted(&mut self, v: impl IntoIterator<Item = f32>) {
        self.merge_sorted_centroids(Self::vals_to_centroids(v).into_iter().collect());
        self.cluster_centroids();
    }
}