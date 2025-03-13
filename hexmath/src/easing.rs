// Easing functions.

pub trait Easing {
    fn quadratic_in(self) -> Self;
    fn quadratic_out(self) -> Self;
    fn quadratic_in_out(self) -> Self;
    fn cubic_in(self) -> Self;
    fn cubic_out(self) -> Self;
    fn cubic_in_out(self) -> Self;
    fn quartic_in(self) -> Self;
    fn quartic_out(self) -> Self;
    fn quartic_in_out(self) -> Self;
    fn quintic_in(self) -> Self;
    fn quintic_out(self) -> Self;
    fn quintic_in_out(self) -> Self;
    fn sine_in(self) -> Self;
    fn sine_out(self) -> Self;
    fn sine_in_out(self) -> Self;
    fn circular_in(self) -> Self;
    fn circular_out(self) -> Self;
    fn circular_in_out(self) -> Self;
    fn exp_in(self) -> Self;
    fn exp_out(self) -> Self;
    fn exp_in_out(self) -> Self;
}

macro_rules! easing_functions {
    ($type:ident) => {
        impl Easing for $type {
            #[inline]
            fn quadratic_in(self) -> Self {
                self * self
            }
        
            #[inline]
            fn quadratic_out(self) -> Self {
                self * (2.0 - self)
            }
        
            #[inline]
            fn quadratic_in_out(self) -> Self {
                if self < 0.5 {
                    2.0 * self * self
                } else {
                    -1.0 + (4.0 - 2.0 * self) * self
                }
            }
        
            #[inline]
            fn cubic_in(self) -> Self {
                self.powf(3.0)
            }
        
            #[inline]
            fn cubic_out(self) -> Self {
                let t1 = self - 1.0;
                t1 * t1 * t1 + 1.0
            }
        
            #[inline]
            fn cubic_in_out(self) -> Self {
                if self < 0.5 {
                    4.0 * self * self * self
                } else {
                    let t1 = (2.0 * self) - 2.0;
                    0.5 * t1 * t1 * t1 + 1.0
                }
            }
        
            #[inline]
            fn quartic_in(self) -> Self {
                self.powf(4.0)
            }
        
            #[inline]
            fn quartic_out(self) -> Self {
                let t1 = self - 1.0;
                1.0 - t1 * t1 * t1 * t1
            }
        
            #[inline]
            fn quartic_in_out(self) -> Self {
                if self < 0.5 {
                    8.0 * self * self * self * self
                } else {
                    let t1 = self - 1.0;
                    1.0 - 8.0 * t1 * t1 * t1 * t1
                }
            }
        
            #[inline]
            fn quintic_in(self) -> Self {
                self.powf(5.0)
            }
        
            #[inline]
            fn quintic_out(self) -> Self {
                let t1 = self - 1.0;
                1.0 + t1 * t1 * t1 * t1 * t1
            }
        
            #[inline]
            fn quintic_in_out(self) -> Self {
                if self < 0.5 {
                    16.0 * self * self * self * self * self
                } else {
                    let t1 = (2.0 * self) - 2.0;
                    0.5 * t1 * t1 * t1 * t1 * t1 + 1.0
                }
            }
        
            #[inline]
            fn sine_in(self) -> Self {
                1.0 - (self * std::$type::consts::FRAC_PI_2).cos()
            }
        
            #[inline]
            fn sine_out(self) -> Self {
                (self * std::$type::consts::FRAC_PI_2).sin()
            }
        
            #[inline]
            fn sine_in_out(self) -> Self {
                0.5 * (1.0 - (std::$type::consts::PI * self).cos())
            }
        
            #[inline]
            fn circular_in(self) -> Self {
                1.0 - (1.0 - self.powf(2.0)).sqrt()
            }
        
            #[inline]
            fn circular_out(self) -> Self {
                (1.0 - (1.0 - self).powf(2.0)).sqrt()
            }
        
            #[inline]
            fn circular_in_out(self) -> Self {
                if self < 0.5 {
                    0.5 * (1.0 - (1.0 - (4.0 * self.powf(2.0))).sqrt())
                } else {
                    0.5 * ((1.0 - 4.0 * (1.0 - self).powf(2.0)).sqrt() + 1.0)
                }
            }
        
            #[inline]
            fn exp_in(self) -> Self {
                if self == 0.0 {
                    0.0
                } else {
                    (2.0 as Self).powf(10.0 * (self - 1.0))
                }
            }
        
            #[inline]
            fn exp_out(self) -> Self {
                if self == 1.0 {
                    1.0
                } else {
                    1.0 - (2.0 as Self).powf(-10.0 * self)
                }
            }
        
            #[inline]
            fn exp_in_out(self) -> Self {
                if self == 0.0 {
                    0.0
                } else if self == 1.0 {
                    1.0
                } else if self < 0.5 {
                    0.5 * (2.0 as Self).powf(20.0 * self - 10.0)
                } else {
                    1.0 - 0.5 * (2.0 as Self).powf(-20.0 * self + 10.0)
                }
            }
        }


        pub mod $type {
            #[inline]
            pub fn quadratic_in(t: $type) -> $type {
                t * t
            }
            
            #[inline]
            pub fn quadratic_out(t: $type) -> $type {
                t * (2.0 - t)
            }
            
            #[inline]
            pub fn quadratic_in_out(t: $type) -> $type {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            
            #[inline]
            pub fn cubic_in(t: $type) -> $type {
                t.powf(3.0)
            }
            
            #[inline]
            pub fn cubic_out(t: $type) -> $type {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            
            #[inline]
            pub fn cubic_in_out(t: $type) -> $type {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t1 = (2.0 * t) - 2.0;
                    0.5 * t1 * t1 * t1 + 1.0
                }
            }
            
            #[inline]
            pub fn quartic_in(t: $type) -> $type {
                t.powf(4.0)
            }
            
            #[inline]
            pub fn quartic_out(t: $type) -> $type {
                let t1 = t - 1.0;
                1.0 - t1 * t1 * t1 * t1
            }
            
            #[inline]
            pub fn quartic_in_out(t: $type) -> $type {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let t1 = t - 1.0;
                    1.0 - 8.0 * t1 * t1 * t1 * t1
                }
            }
            
            #[inline]
            pub fn quintic_in(t: $type) -> $type {
                t.powf(5.0)
            }
            
            #[inline]
            pub fn quintic_out(t: $type) -> $type {
                let t1 = t - 1.0;
                1.0 + t1 * t1 * t1 * t1 * t1
            }
            
            #[inline]
            pub fn quintic_in_out(t: $type) -> $type {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    let t1 = (2.0 * t) - 2.0;
                    0.5 * t1 * t1 * t1 * t1 * t1 + 1.0
                }
            }
            
            #[inline]
            pub fn sine_in(t: $type) -> $type {
                1.0 - (t * std::$type::consts::FRAC_PI_2).cos()
            }
            
            #[inline]
            pub fn sine_out(t: $type) -> $type {
                (t * std::$type::consts::FRAC_PI_2).sin()
            }
            
            #[inline]
            pub fn sine_in_out(t: $type) -> $type {
                0.5 * (1.0 - (std::$type::consts::PI * t).cos())
            }
        
            #[inline]
            pub fn circular_in(t: $type) -> $type {
                1.0 - (1.0 - t.powf(2.0)).sqrt()
            }
        
            #[inline]
            pub fn circular_out(t: $type) -> $type {
                (1.0 - (1.0 - t).powf(2.0)).sqrt()
            }
        
            #[inline]
            pub fn circular_in_out(t: $type) -> $type {
                if t < 0.5 {
                    0.5 * (1.0 - (1.0 - (4.0 * t.powf(2.0))).sqrt())
                } else {
                    0.5 * ((1.0 - 4.0 * (1.0 - t).powf(2.0)).sqrt() + 1.0)
                }
            }
        
            #[inline]
            pub fn exp_in(t: $type) -> $type {
                if t == 0.0 {
                    0.0
                } else {
                    (2.0 as $type).powf(10.0 * (t - 1.0))
                }
            }
        
            #[inline]
            pub fn exp_out(t: $type) -> $type {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - (2.0 as $type).powf(-10.0 * t)
                }
            }
        
            #[inline]
            pub fn exp_in_out(t: $type) -> $type {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    0.5 * (2.0 as $type).powf(20.0 * t - 10.0)
                } else {
                    1.0 - 0.5 * (2.0 as $type).powf(-20.0 * t + 10.0)
                }
            }
        }
    };
}

easing_functions!(f32);
easing_functions!(f64);