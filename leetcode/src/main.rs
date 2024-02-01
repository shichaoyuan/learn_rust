mod add_two_numbers;
mod burst_balloons;
mod car_pooling;
mod combination_sum;
mod combination_sum_ii;
mod combination_sum_iii;
mod contains_duplicate;
mod contains_duplicate_ii;
mod contains_duplicate_iii;
mod corporate_flight_bookings;
mod daily_temperatures;
mod jump_game;
mod jump_game_ii;
mod longest_substring_without_repeating_characters;
mod merge_two_binary_trees;
mod minimum_number_of_arrows_to_burst_balloons;
mod next_greater_element_i;
mod next_greater_element_ii;
mod non_overlapping_intervals;
mod palindromic_substrings;
mod two_sum;
mod valid_palindrome;
mod video_stitching;
mod insert_delete_getrandom_o1;
mod random_pick_with_blacklist;
mod all_paths_from_source_to_target;
mod surrounded_regions;
mod min_cost_to_connect_all_points;
mod subsets;
mod subsets_ii;
mod combinations;

struct UF {
    count: usize,
    parent: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n]; //Vec::with_capacity(n);
        for i in 0..n {
            parent[i] = i;
        }
        UF {
            count: n,
            parent,
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);
        if root_p != root_q {
            self.parent[root_q] = root_p;
            self.count -= 1;
        }
    }

    fn connected(&mut self, p: usize, q: usize) -> bool {
        let root_p = self.find(p);
        let root_q = self.find(q);
        root_p == root_q
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}


fn main() {
    println!("hello leetcode");
}
