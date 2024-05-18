use crate::{matchup::Matchup, season::Season};

pub struct TreeNode {
    away_score: i8,
    home_score: i8,
    // in_ot: bool,
    leaves: Option< (Vec<f64>,Vec<TreeNode>) >
}
pub struct TreeDistr {
    pub away_scores: Vec<f64>,
    pub home_scores: Vec<f64>,
    pub over_under: Vec<f64>,
    pub margin: Vec<f64>,
    pub over_under_max: i8,
    pub margin_min: i8,
    pub margin_max: i8
}
impl TreeNode {
    pub fn get_away_score(&self) -> i8 {
        return self.away_score;
    }
    pub fn get_home_score(&self) -> i8 {
        return self.home_score;
    }
    
    pub fn add_leaves(&mut self,probs: Vec<f64>,leaves: Vec<TreeNode>) {
        self.leaves = Some( (probs,leaves) );
    }
    // pub fn mark_as_ot(&mut self) {
    //     self.in_ot = true;
    // }

    pub fn get_child_count(&self) -> usize {
        return match &self.leaves {
            None => 1,
            Some((_,leaves)) => {
                let mut total = 1usize;
                for leaf in leaves {
                    total += leaf.get_child_count();
                }
                total
            }
        }
    }

    pub fn get_exp_drives(&self) -> f64 {
        return self.summer(1.0,0,|_,level| level);
    }
    pub fn get_exp_away_score(&self) -> f64 {
        return self.summer(1.0,0,|leaf,_| leaf.away_score);
    }
    pub fn get_exp_home_score(&self) -> f64 {
        return self.summer(1.0,0,|leaf,_| leaf.home_score);
    }
    fn summer(&self,running_prob: f64,level: i8,base: fn(node: &TreeNode,level: i8) -> i8) -> f64 {
        return match &self.leaves {
            None => running_prob * (base(&self,level) as f64),
            Some((probs,leaves)) => {
                let mut total = 0f64;
                for (prob,leaf) in probs.iter().zip(leaves.iter()) {
                    if prob == &0.0 {
                        continue;
                    }
                    // if leaf.in_ot && !include_ot {
                    //     total += (base(self,level) as f64) * running_prob * prob
                    // } else {
                    // }
                    total += leaf.summer(running_prob*prob,level+1,base);
                }
                total
            }
        }
    }
    
    pub fn get_max_away_score(&self) -> i8 {
        return self.extrema(true,|leaf| leaf.away_score);
    }
    pub fn get_max_home_score(&self) -> i8 {
        return self.extrema(true,|leaf| leaf.home_score);
    }
    pub fn get_max_total_score(&self) -> i8 {
        return self.extrema(true,|leaf| leaf.away_score + leaf.home_score);
    }
    pub fn get_max_margin_score(&self) -> i8 {
        return self.extrema(true,|leaf| leaf.home_score - leaf.away_score);
    }
    pub fn get_min_margin_score(&self) -> i8 {
        return self.extrema(false,|leaf| leaf.home_score - leaf.away_score);
    }
    fn extrema(&self,maximize: bool,base: fn(node: &TreeNode) -> i8) -> i8 {
        return match &self.leaves {
            None => base(self),
            Some((probs,leaves)) => {
                let mut extrema = 0i8;
                for (prob,leaf) in probs.iter().zip(leaves.iter()) {
                    if prob == &0.0 {
                        continue;
                    }
                    if maximize {
                        extrema = extrema.max(leaf.extrema(maximize,base));
                    } else {
                        extrema = extrema.min(leaf.extrema(maximize,base));
                    }
                }
                extrema
            }
        }
    }

    pub fn get_distributions(&self) -> TreeDistr {
        let away_scores = self.build_away_distr();
        let home_scores = self.build_home_distr();

        let over_under_max = self.get_max_total_score();
        let over_under_length = (self.get_max_total_score()+1) as usize;

        let mut over_under: Vec<f64> = vec![0.0;over_under_length];
        for (away_points,away_prob) in away_scores.iter().enumerate() {
            for (home_points,home_prob) in home_scores.iter().enumerate() {
                if away_points + home_points >= over_under_length {
                    continue;
                }
                over_under[away_points+home_points] += away_prob * home_prob;
            }
        }

        let margin_max = self.get_max_margin_score();
        let margin_min = self.get_min_margin_score();

        let margin_length = (margin_max - margin_min + 1) as usize;
        let mut margin: Vec<f64> = vec![0.0;margin_length];
        // println!("margin max {} margin min {} margin len {}",margin_max,margin_min,margin_length);
        for (away_points,away_prob) in away_scores.iter().enumerate() {
            for (home_points,home_prob) in home_scores.iter().enumerate() {
                let iter_margin = home_points - away_points;
                let margin_index = ((iter_margin as i8) - margin_min) as usize;
                if margin_index >= margin_length {
                    continue;
                }
                margin[margin_index] += home_prob * away_prob;
            }
        } 

        return TreeDistr {
            away_scores,
            home_scores,
            margin,
            over_under,
            over_under_max,
            margin_max,
            margin_min
        };
    }
    pub fn build_away_distr(&self) -> Vec<f64> {
        let length = self.get_max_away_score() as usize;
        let mut distr = vec![0.0;length+1];
        self.build_distr(&mut distr, 1.0,true);
        return distr;
    }
    pub fn build_home_distr(&self) -> Vec<f64> {
        let length = self.get_max_home_score() as usize;
        let mut distr = vec![0.0;length+1];
        self.build_distr(&mut distr,1.0,false);
        return distr;
    }
    fn build_distr(&self,distr: &mut [f64],cumul: f64,is_away: bool) {
        if let Some((probs,leaves)) = &self.leaves {
            for (prob,leaf) in probs.iter().zip(leaves.iter()) {
                if prob == &0.0 {
                    continue;
                }
                leaf.build_distr(distr,cumul*prob,is_away);
            }
        } else {
            distr[if is_away {
                self.away_score as usize
            } else {
                self.home_score as usize
            }] += cumul;
        }
    }
    
}

// const I_TOUCHDOWN: usize = 0;
// const I_FIELD_GOAL: usize = 1;
// const I_SAFETY: usize = 2;
// const I_TURNOVER: usize = 2;
// const I_PUNT: usize = 4;
const I_END_OF_PERIOD: usize = 3;

// const OFFENSE_POINTS: [i8;5] = [7,3,0,0,0];
// const DEFENSE_POINTS: [i8;5] = [0,0,2,0,0];
const OFFENSE_POINTS: [i8;4] = [7,3,0,0];
const DEFENSE_POINTS: [i8;4] = [0,0,0,0];

// const TURNOVER_TD_BOOST: f64 = 1.5;
// const TURNOVER_FG_BOOST: f64 = 1.5;

fn half_end_prob(drives: i8) -> f64 { //[7=0.03,8=0.58,9=0.38,10=0.01,11=0]
    // return if drives >= 2 { 1.0 } else { 0.0 };
    const OFFSET: i8 = 1;
    return if drives < 7 + OFFSET {
        0.0
    } else if drives == 7 + OFFSET {
        0.03 //0.03
    } else if drives == 8 + OFFSET {
        0.6 //0.58
    } else if drives == 9 + OFFSET {
        0.974
    } else {
        1.0
    };
}

fn average(a: [f64;4],b: [f64;4]) -> [f64;4] {
    return [
        (a[0]+b[0]) / 2.0,
        (a[1]+b[1]) / 2.0,
        (a[2]+b[2]) / 2.0,
        (a[3]+b[3]) / 2.0,
        // (a[4]+b[4]) / 2.0,
        // (a[5]+b[5]) / 2.0,
    ];
}

fn normalize(a: [f64;4]) -> [f64;4] {
    let sum = a.iter().sum::<f64>();
    return [
        a[0] / sum,
        a[1] / sum,
        a[2] / sum,
        a[3] / sum,
        // a[4] / sum,
        // a[5] / sum,
    ];
}

pub fn game_tree_builder(predict: bool,away_off: &Season,away_def: &Season,home_off: &Season,home_def: &Season) -> TreeNode {
    let away_off_probs = if !predict { away_off.get_distribution() } else { away_off.predict_off_distr() };
    let home_off_probs = if !predict { home_off.get_distribution() } else { home_off.predict_off_distr() };
    let away_def_probs = if !predict { away_def.get_distribution() } else { away_def.predict_def_distr() };
    let home_def_probs = if !predict { home_def.get_distribution() } else { home_def.predict_def_distr() };
    let away_probs = average(away_off_probs,home_def_probs);
    let home_probs = average(home_off_probs,away_def_probs);

    fn recursive_builder(mut root: TreeNode,away_has_ball: bool,drive: i8,away_probs: &[f64;4],home_probs: &[f64;4]) -> TreeNode {
        if drive > 15 {
            panic!("wtf");
        }
        
        let end = half_end_prob(drive);
        // let force_end_period = end == 1.0;
        let mut base_distr = if end == 1.0 {
            return root;
        } else {
            (if away_has_ball { away_probs } else { home_probs }).clone()
        };
        base_distr[I_END_OF_PERIOD] *= end;

        let prob = normalize(base_distr);

        let away_points = if away_has_ball { OFFENSE_POINTS } else { DEFENSE_POINTS };
        let home_points = if away_has_ball { DEFENSE_POINTS } else { OFFENSE_POINTS };

        let away_score = root.get_away_score();
        let home_score = root.get_home_score();

        let mut leaves: Vec<TreeNode> = vec![];

        for i in 0..=I_END_OF_PERIOD {
            let leaf = TreeNode {
                away_score: away_score + away_points[i],
                home_score: home_score + home_points[i],
                // in_ot,
                leaves: None
            };

            if prob[i] == 0.0 || (i == I_END_OF_PERIOD && prob[i] == 1.0) || i == I_END_OF_PERIOD {
                leaves.push(leaf);
            }
            // else if i == I_END_OF_PERIOD { //no points added on either side
                // if !in_ot && away_score == home_score { //start OT
                //     leaf.mark_as_ot();
                //     let x: bool = rand::random();
                //     leaves.push( recursive_builder(leaf,x,8,true,away_probs,home_probs) );
                // } else { //end the period
                // }
                // leaves.push(leaf);
            // }
            // else if in_ot && away_score != home_score {
            //     leaves.push(leaf);
            // }
            else {
                leaves.push( recursive_builder(leaf,!away_has_ball,drive+1,away_probs,home_probs) );
            }
        }
        
        root.add_leaves(prob.to_vec(),leaves);

        return root;
    }

    // const AWAY_START: i8 = 1;
    // const HOME_START: i8 = 2;
    let away_node = recursive_builder(TreeNode {
        away_score: 0,
        home_score: 1,
        // in_ot: false,
        leaves: None
    },true,1,&away_probs,&home_probs);
    let home_node = recursive_builder(TreeNode {
        away_score: 1,
        home_score: 2,
        // in_ot: false,
        leaves: None
    },false,1,&away_probs,&home_probs);

    let mut parent_node = TreeNode {
        away_score: 0,
        home_score: 0,
        // in_ot: false,
        leaves: None
    };
    parent_node.add_leaves(vec![0.5,0.5],vec![away_node,home_node]);

    return parent_node;
}