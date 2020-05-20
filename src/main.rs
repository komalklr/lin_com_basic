use bulletproofs::r1cs::{Verifier,ConstraintSystem,LinearCombination,R1CSError,R1CSProof};
use bulletproofs::{BulletproofGens, PedersenGens};
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use merlin::Transcript;
use std::fs;
use std::str::FromStr;

fn example_gadget<CS: ConstraintSystem>(
    cs: &mut CS,
    a: LinearCombination,
    b: LinearCombination,
    c: LinearCombination,
) {
    let (_, _, c_var) = cs.multiply(a, b);
    cs.constrain(c - c_var);
}
fn main() {
    let proof: String = fs::read_to_string("proof.txt").expect("proof.txt is not found in current directory.");
    let commitments:String = fs::read_to_string("commitments.txt").expect("commitments.txt is not found in current directory.");
    let c:String = fs::read_to_string("ll.txt").expect("ll.txt is not found in current directory.");
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(2, 1);
    let proof:String=proof[0..834].to_string();
   // println!("proof..{}",proof);
   
    let proof=R1CSProof::from_bytes(&hex::decode(proof).unwrap())
                .expect("Rangeproof deserialization failed");
    let mut split_commitments = commitments.split(" ");
    let mut v: Vec<&str> = split_commitments.collect();
   // println!("com1..{}{}",v[0].len(),v[0]);
   // println!("com2..{}{}",v[1].len(),v[1]);
    let mut commitments=Vec::new();
    commitments.push(CompressedRistretto::from_slice(
        &hex::decode(v[0].to_string())
            .unwrap()));
    commitments.push(CompressedRistretto::from_slice(
        &hex::decode(v[1][0..64].to_string())
            .unwrap()));
    let c = i64::from_str(&(c[0..c.len()-1])).unwrap() as u64;
    let mut transcript = Transcript::new(b"R1CS");
    let mut verifier = Verifier::new(&mut transcript);
    let vars: Vec<_> = commitments.iter().map(|V| verifier.commit(*V)).collect();
    example_gadget(
        &mut verifier,
        vars[0].into(),
        vars[1].into(),
        Scalar::from(c).into(),
    );
    let result = verifier
        .verify(&proof, &pc_gens, &bp_gens)
        .map_err(|_| R1CSError::VerificationError);
    match result {
        Ok(_) => fs::write("out_result.txt", "1").expect("Could not write out_result.txt"),
        Err(_) => fs::write("out_result.txt", "0").expect("Could not write out_result.txt"),
    }   

}
