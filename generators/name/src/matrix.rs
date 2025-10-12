/// assumes row-major order: M =
/// [a,b,c],
/// [d,e,f],
/// [g,h,i]
/// and M[0] = [a,b,c] NOT [a,d,g]
/// also assumes no NaN / inf or that the user doesn't care
fn mmul<const N: usize>(mi0: &[[f32; N]; N], mi1: &[[f32; N]; N]) -> [[f32; N]; N] {
    let mut mo = [[0.; N]; N];

    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                mo[i][j] += mi0[i][k] * mi1[k][j];
            }
        }
    }
    return mo;
}

fn id<const N: usize>() -> [[f32; N]; N] {
    let mut id = [[0.; N]; N];
    for i in 0..N {
        id[i][i] = 1.;
    }
    id
}

fn mpow<const N: usize>(mi: &[[f32; N]; N], n: usize) -> [[f32; N]; N] {
    if n == 0 {
        return id();
    }
    if n == 1 {
        return *mi;
    }
    let mut M = mi.clone();

    for _ in 0..(n - 1) {
        M = mmul(&M, mi);
    }
    return M;
}

fn argmaxf32(slice: &[f32]) -> Option<usize> {
    slice
        .iter()
        .enumerate()
        .max_by(|(_, value0), (_, value1)| value0.total_cmp(value1))
        .map(|x| x.0)
}

#[cfg(test)]
mod testing {
    use super::{id, mpow};

    #[test]
    fn test_mpow() {
        let identity = id::<32>();
        let rot_z_pi_half = [[0f32, -1., 0.], [1., 0., 0.], [0., 0., 1.]];

        for i in 0..4 {
            assert_eq!(mpow(&identity, i), identity);
        }
        assert_eq!(mpow(&rot_z_pi_half, 0), id());

        assert_eq!(mpow(&rot_z_pi_half, 4), id());
    }

    // #[test]
    // fn test_mpow_markov(){
    //     let mkv: MarkovModel<3> = MarkovModel::new(include_str!("../names.txt"));
    //     for i in 2..=10{
    //         let power = mpow(&mkv.probabilities, i);
    //         // println!("Power: {i}");
    //         // for l in power{
    //         //     println!("{:.2?}", l)
    //         // }
    //     }
    // }
}
