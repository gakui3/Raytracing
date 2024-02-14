use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use rand::prelude::*;
/// Debug,Copy,Clone,PartialEqという機能を持ったpublicなFloat3という構造体を定義している
/// Debug:構造体のインスタンスをデバッガで見やすい形式で出力できるようにする。例えば、println!("{:?}", instance); としてインスタンスの内容を確認できる
/// Copy:このトレイトが実装されていると、構造体のインスタンスは「値によるコピー」（ビット単位のコピー）が可能になる。つまり、インスタンスを別の変数に代入すると、そのデータのコピーが作成される
/// Clone:インスタンスを複製するためのメソッド（.clone()）を提供する
/// PartialEq:== と != 演算子を使用して、インスタンス間の等価性比較を行うことができるようになる
#[derive(Debug, Copy, Clone, PartialEq)]

//ここで[f64; 3]にpubをつけることで、Float3構造体の内部の配列が外部からアクセス可能になる
//実装例：let v = Float3([1.0, 2.0, 3.0]); println!("{:?}", v.0[0]); // 1.0
pub struct Float3(pub [f64; 3]);

/// type:型エイリアスという。型に別名をつけることができる
/// たとえば、Color型をFloat3型と同じものとして定義している
/// これらのエイリアスは、CopyやCloneなどのトレイトを実装しているため、Float3型と同じように扱うことができる
pub type Color = Float3;
pub type Vector3 = Float3;
pub type Point3 = Float3;

impl Float3 {
    ///コンストラクタ関数の定義
    pub const fn new(x: f64, y: f64, z: f64) -> Float3 {
        Self([x, y, z])
    }

    pub const fn zero() -> Float3 {
        Self([0.0, 0.0, 0.0])
    }

    pub const fn one() -> Float3 {
        Self([1.0, 1.0, 1.0])
    }

    pub const fn full(value: f64) -> Float3 {
        Self([value; 3])
    }

    /// Float3の各要素を取得するメソッド
    // pub fn length(&self) -> f64 {
    //     let x = self.0[0];
    //     let y = self.0[1];
    //     let z = self.0[2];

    //     (x * x + y * y + z * z).sqrt()
    // }

    // 別の Float3 との和を計算するメソッド
    pub fn add(&self, other: Float3) -> Float3 {
        Float3([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }

    pub fn sqrt(&self) -> Self {
        Self::from_iter(self.0.iter().map(|x| x.sqrt()))
    }
    // pub fn near_zero(&self) -> bool {
    //     self.0.iter().all(|x| x.abs() < EPS)
    // }

    pub fn saturate(&self) -> Self {
        Self::from_iter(self.0.iter().map(|x| x.min(1.0).max(0.0)))
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.0
            .iter()
            .zip(rhs.0.iter())
            .fold(0.0, |acc, (l, r)| acc + l * r)
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self([
            self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
            self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
            self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
        ])
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0.iter().fold(0.0, |acc, x| acc + x * x)
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn lerp(&self, v: Self, t: f64) -> Self {
        *self + (v - *self) * t
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }
    pub const fn xaxis() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    pub const fn yaxis() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    pub const fn zaxis() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn random() -> Self {
        Self::new(random::<f64>(), random::<f64>(), random::<f64>())
    }

    pub fn random_full() -> Self {
        Self::full(random::<f64>())
    }

    pub fn random_limit(min: f64, max: f64) -> Self {
        Self::from_iter(Self::random().0.iter().map(|x| min + x * (max - min)))
    }

    pub fn at(origin: Vector3, direction: Point3) -> Point3 {
        origin + direction * 2.0
    }

    pub fn random_in_unit_sphere() -> Float3 {
        loop {
            let point = Self::random_limit(-1.0, 1.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    pub fn gamma(&self, factor: f64) -> Float3 {
        let recip = factor.recip();
        Self::from_iter(self.0.iter().map(|x| x.powf(recip)))
    }

    pub fn reflect(&self, normal: Self) -> Float3 {
        *self - normal * 2.0 * self.dot(normal)
    }
}

impl FromIterator<f64> for Float3 {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let mut initer = iter.into_iter();
        Float3([
            initer.next().unwrap(),
            initer.next().unwrap(),
            initer.next().unwrap(),
        ])
    }
}

impl Div<f64> for Float3 {
    type Output = Float3;

    fn div(self, rhs: f64) -> Self::Output {
        Float3([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

impl Add for Float3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Float3([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

impl Sub for Float3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Float3([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ])
    }
}

impl Mul<f64> for Float3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Float3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

impl Mul for Float3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Float3([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}

impl Neg for Float3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::from_iter(self.0.iter().map(|x| -x))
    }
}

impl AddAssign<Float3> for Float3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.0[i] += rhs.0[i]
        }
    }
}
