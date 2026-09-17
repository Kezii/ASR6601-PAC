#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "pin0 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af0 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af0> for u8 {
    #[inline(always)]
    fn from(variant: Af0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af0 {
    type Ux = u8;
}
impl crate::IsEnum for Af0 {}
#[doc = "Field `AF0` reader - pin0 function selection"]
pub type Af0R = crate::FieldReader<Af0>;
impl Af0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af0> {
        match self.bits {
            0 => Some(Af0::Fun0),
            1 => Some(Af0::Fun1),
            2 => Some(Af0::Fun2),
            3 => Some(Af0::Fun3),
            4 => Some(Af0::Fun4),
            5 => Some(Af0::Fun5),
            6 => Some(Af0::Fun6),
            7 => Some(Af0::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af0::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af0::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af0::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af0::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af0::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af0::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af0::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af0::Fun7
    }
}
#[doc = "Field `AF0` writer - pin0 function selection"]
pub type Af0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af0>;
impl<'a, REG> Af0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af0::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af0::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af0::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af0::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af0::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af0::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af0::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af0::Fun7)
    }
}
#[doc = "pin1 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af1 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af1> for u8 {
    #[inline(always)]
    fn from(variant: Af1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af1 {
    type Ux = u8;
}
impl crate::IsEnum for Af1 {}
#[doc = "Field `AF1` reader - pin1 function selection"]
pub type Af1R = crate::FieldReader<Af1>;
impl Af1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af1> {
        match self.bits {
            0 => Some(Af1::Fun0),
            1 => Some(Af1::Fun1),
            2 => Some(Af1::Fun2),
            3 => Some(Af1::Fun3),
            4 => Some(Af1::Fun4),
            5 => Some(Af1::Fun5),
            6 => Some(Af1::Fun6),
            7 => Some(Af1::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af1::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af1::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af1::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af1::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af1::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af1::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af1::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af1::Fun7
    }
}
#[doc = "Field `AF1` writer - pin1 function selection"]
pub type Af1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af1>;
impl<'a, REG> Af1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af1::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af1::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af1::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af1::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af1::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af1::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af1::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af1::Fun7)
    }
}
#[doc = "pin2 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af2 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af2> for u8 {
    #[inline(always)]
    fn from(variant: Af2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af2 {
    type Ux = u8;
}
impl crate::IsEnum for Af2 {}
#[doc = "Field `AF2` reader - pin2 function selection"]
pub type Af2R = crate::FieldReader<Af2>;
impl Af2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af2> {
        match self.bits {
            0 => Some(Af2::Fun0),
            1 => Some(Af2::Fun1),
            2 => Some(Af2::Fun2),
            3 => Some(Af2::Fun3),
            4 => Some(Af2::Fun4),
            5 => Some(Af2::Fun5),
            6 => Some(Af2::Fun6),
            7 => Some(Af2::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af2::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af2::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af2::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af2::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af2::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af2::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af2::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af2::Fun7
    }
}
#[doc = "Field `AF2` writer - pin2 function selection"]
pub type Af2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af2>;
impl<'a, REG> Af2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af2::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af2::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af2::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af2::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af2::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af2::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af2::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af2::Fun7)
    }
}
#[doc = "pin3 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af3 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af3> for u8 {
    #[inline(always)]
    fn from(variant: Af3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af3 {
    type Ux = u8;
}
impl crate::IsEnum for Af3 {}
#[doc = "Field `AF3` reader - pin3 function selection"]
pub type Af3R = crate::FieldReader<Af3>;
impl Af3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af3> {
        match self.bits {
            0 => Some(Af3::Fun0),
            1 => Some(Af3::Fun1),
            2 => Some(Af3::Fun2),
            3 => Some(Af3::Fun3),
            4 => Some(Af3::Fun4),
            5 => Some(Af3::Fun5),
            6 => Some(Af3::Fun6),
            7 => Some(Af3::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af3::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af3::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af3::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af3::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af3::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af3::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af3::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af3::Fun7
    }
}
#[doc = "Field `AF3` writer - pin3 function selection"]
pub type Af3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af3>;
impl<'a, REG> Af3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af3::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af3::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af3::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af3::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af3::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af3::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af3::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af3::Fun7)
    }
}
#[doc = "pin4 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af4 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af4> for u8 {
    #[inline(always)]
    fn from(variant: Af4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af4 {
    type Ux = u8;
}
impl crate::IsEnum for Af4 {}
#[doc = "Field `AF4` reader - pin4 function selection"]
pub type Af4R = crate::FieldReader<Af4>;
impl Af4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af4> {
        match self.bits {
            0 => Some(Af4::Fun0),
            1 => Some(Af4::Fun1),
            2 => Some(Af4::Fun2),
            3 => Some(Af4::Fun3),
            4 => Some(Af4::Fun4),
            5 => Some(Af4::Fun5),
            6 => Some(Af4::Fun6),
            7 => Some(Af4::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af4::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af4::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af4::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af4::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af4::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af4::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af4::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af4::Fun7
    }
}
#[doc = "Field `AF4` writer - pin4 function selection"]
pub type Af4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af4>;
impl<'a, REG> Af4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af4::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af4::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af4::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af4::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af4::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af4::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af4::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af4::Fun7)
    }
}
#[doc = "pin5 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af5 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af5> for u8 {
    #[inline(always)]
    fn from(variant: Af5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af5 {
    type Ux = u8;
}
impl crate::IsEnum for Af5 {}
#[doc = "Field `AF5` reader - pin5 function selection"]
pub type Af5R = crate::FieldReader<Af5>;
impl Af5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af5> {
        match self.bits {
            0 => Some(Af5::Fun0),
            1 => Some(Af5::Fun1),
            2 => Some(Af5::Fun2),
            3 => Some(Af5::Fun3),
            4 => Some(Af5::Fun4),
            5 => Some(Af5::Fun5),
            6 => Some(Af5::Fun6),
            7 => Some(Af5::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af5::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af5::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af5::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af5::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af5::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af5::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af5::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af5::Fun7
    }
}
#[doc = "Field `AF5` writer - pin5 function selection"]
pub type Af5W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af5>;
impl<'a, REG> Af5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af5::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af5::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af5::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af5::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af5::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af5::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af5::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af5::Fun7)
    }
}
#[doc = "pin6 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af6 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af6> for u8 {
    #[inline(always)]
    fn from(variant: Af6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af6 {
    type Ux = u8;
}
impl crate::IsEnum for Af6 {}
#[doc = "Field `AF6` reader - pin6 function selection"]
pub type Af6R = crate::FieldReader<Af6>;
impl Af6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af6> {
        match self.bits {
            0 => Some(Af6::Fun0),
            1 => Some(Af6::Fun1),
            2 => Some(Af6::Fun2),
            3 => Some(Af6::Fun3),
            4 => Some(Af6::Fun4),
            5 => Some(Af6::Fun5),
            6 => Some(Af6::Fun6),
            7 => Some(Af6::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af6::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af6::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af6::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af6::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af6::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af6::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af6::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af6::Fun7
    }
}
#[doc = "Field `AF6` writer - pin6 function selection"]
pub type Af6W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af6>;
impl<'a, REG> Af6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af6::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af6::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af6::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af6::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af6::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af6::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af6::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af6::Fun7)
    }
}
#[doc = "pin7 function selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Af7 {
    #[doc = "0: Alternate function 0"]
    Fun0 = 0,
    #[doc = "1: Alternate function 1"]
    Fun1 = 1,
    #[doc = "2: Alternate function 2"]
    Fun2 = 2,
    #[doc = "3: Alternate function 3"]
    Fun3 = 3,
    #[doc = "4: Alternate function 4"]
    Fun4 = 4,
    #[doc = "5: Alternate function 5"]
    Fun5 = 5,
    #[doc = "6: Alternate function 6"]
    Fun6 = 6,
    #[doc = "7: Alternate function 7"]
    Fun7 = 7,
}
impl From<Af7> for u8 {
    #[inline(always)]
    fn from(variant: Af7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Af7 {
    type Ux = u8;
}
impl crate::IsEnum for Af7 {}
#[doc = "Field `AF7` reader - pin7 function selection"]
pub type Af7R = crate::FieldReader<Af7>;
impl Af7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Af7> {
        match self.bits {
            0 => Some(Af7::Fun0),
            1 => Some(Af7::Fun1),
            2 => Some(Af7::Fun2),
            3 => Some(Af7::Fun3),
            4 => Some(Af7::Fun4),
            5 => Some(Af7::Fun5),
            6 => Some(Af7::Fun6),
            7 => Some(Af7::Fun7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_fun0(&self) -> bool {
        *self == Af7::Fun0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_fun1(&self) -> bool {
        *self == Af7::Fun1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_fun2(&self) -> bool {
        *self == Af7::Fun2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_fun3(&self) -> bool {
        *self == Af7::Fun3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_fun4(&self) -> bool {
        *self == Af7::Fun4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_fun5(&self) -> bool {
        *self == Af7::Fun5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_fun6(&self) -> bool {
        *self == Af7::Fun6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_fun7(&self) -> bool {
        *self == Af7::Fun7
    }
}
#[doc = "Field `AF7` writer - pin7 function selection"]
pub type Af7W<'a, REG> = crate::FieldWriter<'a, REG, 4, Af7>;
impl<'a, REG> Af7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn fun0(self) -> &'a mut crate::W<REG> {
        self.variant(Af7::Fun0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn fun1(self) -> &'a mut crate::W<REG> {
        self.variant(Af7::Fun1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn fun2(self) -> &'a mut crate::W<REG> {
        self.variant(Af7::Fun2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn fun3(self) -> &'a mut crate::W<REG> {
        self.variant(Af7::Fun3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn fun4(self) -> &'a mut crate::W<REG> {
        self.variant(Af7::Fun4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn fun5(self) -> &'a mut crate::W<REG> {
        self.variant(Af7::Fun5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn fun6(self) -> &'a mut crate::W<REG> {
        self.variant(Af7::Fun6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn fun7(self) -> &'a mut crate::W<REG> {
        self.variant(Af7::Fun7)
    }
}
impl R {
    #[doc = "Bits 0:3 - pin0 function selection"]
    #[inline(always)]
    pub fn af0(&self) -> Af0R {
        Af0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - pin1 function selection"]
    #[inline(always)]
    pub fn af1(&self) -> Af1R {
        Af1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - pin2 function selection"]
    #[inline(always)]
    pub fn af2(&self) -> Af2R {
        Af2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - pin3 function selection"]
    #[inline(always)]
    pub fn af3(&self) -> Af3R {
        Af3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - pin4 function selection"]
    #[inline(always)]
    pub fn af4(&self) -> Af4R {
        Af4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - pin5 function selection"]
    #[inline(always)]
    pub fn af5(&self) -> Af5R {
        Af5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - pin6 function selection"]
    #[inline(always)]
    pub fn af6(&self) -> Af6R {
        Af6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - pin7 function selection"]
    #[inline(always)]
    pub fn af7(&self) -> Af7R {
        Af7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - pin0 function selection"]
    #[inline(always)]
    pub fn af0(&mut self) -> Af0W<'_, AfrlSpec> {
        Af0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - pin1 function selection"]
    #[inline(always)]
    pub fn af1(&mut self) -> Af1W<'_, AfrlSpec> {
        Af1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - pin2 function selection"]
    #[inline(always)]
    pub fn af2(&mut self) -> Af2W<'_, AfrlSpec> {
        Af2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - pin3 function selection"]
    #[inline(always)]
    pub fn af3(&mut self) -> Af3W<'_, AfrlSpec> {
        Af3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - pin4 function selection"]
    #[inline(always)]
    pub fn af4(&mut self) -> Af4W<'_, AfrlSpec> {
        Af4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - pin5 function selection"]
    #[inline(always)]
    pub fn af5(&mut self) -> Af5W<'_, AfrlSpec> {
        Af5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - pin6 function selection"]
    #[inline(always)]
    pub fn af6(&mut self) -> Af6W<'_, AfrlSpec> {
        Af6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - pin7 function selection"]
    #[inline(always)]
    pub fn af7(&mut self) -> Af7W<'_, AfrlSpec> {
        Af7W::new(self, 28)
    }
}
#[doc = "alternate function low register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
}
