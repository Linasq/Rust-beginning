use std::collections::{HashSet, HashMap};


// find how many different nums in array
fn zadanie_1(tab: Vec<i32>) -> i32 {
    if tab.len() == 0 {return 0;}
    let mut nums = HashSet::new();

    for i in tab {
        if !nums.contains(&i){
            nums.insert(i);
        }
    }

    return nums.len() as i32;
}


// find value that is dominant in array (it shows up more times than half of array)
fn zadanie_2(tab: Vec<i32>) -> i32 {
    let mut dict = HashMap::new();
    let length = tab.len()/2;
    
    for i in tab {
        let z = dict.clone().into_keys();
        let mut zamiana = false;
        for zz in z {
            if zz == i {
                *dict.get_mut(&i).unwrap() += 1;
                zamiana = true;
            }
        }
        if !zamiana {
            dict.insert(i, 1);
        }
    }

    for (key, value) in dict.into_iter(){
        if value > length {return key;}
    }

    return 0;
}

// find index, where abs(tab[::x] - tab[x::]) is the closest to 0
fn zadanie_3(tab: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut min = 1234;
    for i in tab.clone() {
        sum += i;
    }

    for i in 0..tab.len() {
        let mut smaller_sum = 0;
        for ii in 0..i {
            smaller_sum += tab[ii]
        }
        let x = (sum - 2 * smaller_sum).abs();
        if x < min {
            min = x;
        }
    }
    return min;
}


fn sd(tab: Vec<i32>) -> f32{
    let sum = tab.iter().sum::<i32>() as f32;
    let length = tab.len() as f32;
    let srednia = sum / length;

    let mut z: f32 = 0.;
    for i in tab {
        let x = i as f32;
        z += (x - srednia) * (x - srednia);
    }
    let mut after = z / length;
    after = after.sqrt() as f32;
    return after;
}

mod tests {
    use super::*;

    #[test]
    fn test_zadanie1(){
        let v = vec![1,4,3,2,2,-1];
        assert_eq!(zadanie_1(v), 5);
    }

    #[test]
    fn test_zadanie1_no_numbers() {
        let v = vec![];
        assert_eq!(zadanie_1(v), 0);
    }

    #[test]
    fn test_zadanie2(){
        let v = vec![4,4,1,4,2,4];
        assert_eq!(zadanie_2(v), 4);
    }


    #[test]
    fn test_zadanie2_v2(){
        let v = vec![5,5,5,5,5,2,2,1];
        assert_eq!(zadanie_2(v), 5);
    }

    #[test]
    fn test_zadanie3() {
        let v = vec![3,1,2,4,3];
        assert_eq!(zadanie_3(v), 1);
    }

    #[test]
    fn test_zadanie3_v2() {
        let v = vec![2,3,3,2];
        assert_eq!(zadanie_3(v), 0);
    }

    #[test]
    fn test_odchylenie_standardowe() {
        let v = vec![1,1,1,1,1,1,1,1,1,1,1];
        assert_eq!(sd(v), 0.);
    }
}

fn main() {}
